import type { TypedRouteLocationRaw, TypedRouteLocationRawFromName } from "@typed-router/__router";
import type { RoutesNamesList, RoutesParamsRecord } from "@typed-router/__routes";
import { enUS } from 'date-fns/locale';
import { differenceInDays, formatRelative, differenceInHours, formatDistance, format } from 'date-fns';
import type { HTMLAttributes } from "vue";

/**
 * Gets the route while including the right locale. It's a thin wrapper around calling
 * `useLocaleRoute` from vue-i18n
 *
 * @param name The route name
 */
export function getRoute(route: TypedRouteLocationRawFromName<RoutesNamesList>): TypedRouteLocationRaw {
	const localeRoute = useLocaleRoute()
	return localeRoute(route);
}

type TNamesWithoutParams = {
	[K in keyof RoutesParamsRecord]: RoutesParamsRecord[K] extends never ? K : never;
}[RoutesNamesList]

/**
 * Gets the route while including the right locale from the given route name.
 *
 * The route `name` must not require params. If it does, use `getRoute` instead.
 *
 * @param name The route name
 */
export function getRouteFromName(name: TNamesWithoutParams): TypedRouteLocationRaw {
	const localeRoute = useLocaleRoute()
	return localeRoute({ name });
}

/**
 * Returns the bytes provided in a human readeable format
 * 
 * @param bytes The number of bytes
 * @returns A string representation of the bytes
 */
export function humanizeBytes(bytes: number): string {
	const gb = bytes / (1024 * 1024 * 1024)
	const mb = (gb - Math.floor(gb)) * 1024
	const kb = (mb - Math.floor(mb)) * 1024

	let suffix = 'B'
	let size = bytes.toString()
	if (gb > 0.8) {
		suffix = 'GB'
		size = gb.toFixed(2)
	} else if (mb > 0.8) {
		suffix = 'MB'
		size = mb.toFixed(2)
	} else if (kb > 0.8) {
		suffix = 'KB'
		size = kb.toFixed(2)
	}

	const sizeSplit = size.split('.')
	const whole = sizeSplit[0]
	let frac = null
	if (sizeSplit.length > 1) frac = sizeSplit[1]

	if (whole.length >= 3) {
		size = whole
	} else if (frac === null || frac === '00') {
		size = whole
	} else if (frac[1] === '0') {
		size = size.substring(0, size.length - 1)
	} else if (whole.length <= 2) {
		size = whole + `.${frac[0]}`
	}

	return `${size} ${suffix}`
}

/**
 * Returns a human friendly version of the date
 *
 * @param date The date to humanize
 */
export function humanizeDate(date: Date, addRelTime = true): string {
	const now = new Date();
	const localeToUse = enUS;

	const daysDiff = differenceInDays(now, date);
	// console.log(date.toISOString(), daysDiff, daysDiff < 2)
	if (daysDiff < 1) {
		let parsedDate = formatRelative(date, now, { locale: localeToUse }).replace(
			/^\w/,
			(v) => v.toUpperCase()
		);
		if (addRelTime && differenceInHours(now, date) <= 6) {
			const relString = formatDistance(date, now, {
				addSuffix: true,
				locale: localeToUse,
			}).replace(/^\w/, (v) => v.toUpperCase());
			parsedDate += ` (${relString})`;
		}

		return parsedDate;
	} else if (daysDiff < 7) {
		return format(date, "eeee 'At' h:mm a");
	} else return format(date, "MMM do, yyyy", { locale: localeToUse });
}


type TAttributes = keyof HTMLAttributes | (string & {});
/**
 * Returns {attrs} with all {attr} removed
 *
 * @param attrs - The record of HTMLAttributes
 * @param attr - The attribute(s) to remove from {attrs}
 */
export function removeFromAttrs(
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	attrs: Record<string, any>,
	attr: TAttributes | TAttributes[],
// eslint-disable-next-line @typescript-eslint/no-explicit-any
): Record<string, any> {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const at: Record<string, any> = {}
	for (const attrib of Object.keys(attrs)) {
		if (Array.isArray(attr) && attr.includes(attrib)) continue
		else if (attrib === attr) continue

		at[attrib] = attrs[attrib]
	}

	return at
}

interface ITweenOptions {
	from?: number
	to?: number
	duration: number
	onUpdate: (value: number, params: { from: number, to: number }, animation: number | null) => void
	onComplete?: () => void
}
/**
 * Performs a tween animation between from and to over duration
 *
 * Returns a function to prematurely cancel the tween 
 */
export function tween({ from = 0, to = 1, duration, onUpdate, onComplete }: ITweenOptions): () => void {
	const startTime = performance.now()

	let animation: number | null = null
	function animate(currentTime: number) {
		const elapsed = currentTime - startTime
		const progress = Math.min(elapsed / duration, 1)
		const value = from + (to - from) * progress

		onUpdate(value, { from, to }, animation)

		if (progress < 1) {
			animation = requestAnimationFrame(animate)
		} else if (onComplete) {
			onComplete()
		}
	}

	animation = requestAnimationFrame(animate)
	return () => {
		if (!animation) return;

		cancelAnimationFrame(animation)
	}
}