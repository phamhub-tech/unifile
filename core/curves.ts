type TCurve = (x: number, max: number) => number

export const Curves = {
	easeOutCubic(x, max = 1) {
		return max - Math.pow(max - x, 3)
	},

	easeOutQuint(x, max = 1) {
		return max - Math.pow(max - x, 5)
	}
} satisfies Record<string, TCurve>

