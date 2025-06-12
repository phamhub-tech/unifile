import type { TypedRouteLocationRaw, TypedRouteLocationRawFromName } from "@typed-router/__router";
import type { RoutesNamesList, RoutesParamsRecord } from "@typed-router/__routes";

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