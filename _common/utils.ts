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