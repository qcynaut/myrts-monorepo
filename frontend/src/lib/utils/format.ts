/**
 * Formats the given duration into a string representation.
 *
 * @param {number} duration - The duration in seconds.
 */
export const formatDurationToString = (duration: number) => {
	const minutes = Math.floor(duration / 60);
	const seconds = duration % 60;

	if (minutes >= 60) {
		const hours = Math.floor(minutes / 60);
		const remainingMinutes = minutes % 60;
		return `${hours} j ${remainingMinutes} m`;
	}

	if (seconds > 0) {
		return `${minutes} m ${seconds} d`;
	}

	return `${minutes} m`;
};

/**
 * Converts a UTC date to local date and returns it as a string.
 *
 * @param {string} date - The UTC date to convert.
 */
export function utcToLocal(date: string): string {
	const utcDate = new Date(date);
	const localDate = new Date(utcDate.getTime() + utcDate.getTimezoneOffset() * 60 * 1000);

	return localDate.toLocaleDateString('id-ID', {
		year: 'numeric',
		month: 'numeric',
		day: 'numeric'
	});
}

/**
 * Formats the given number of seconds into a duration string.
 *
 * @param {number} second - The number of seconds to format.
 * @return {string} The formatted duration string in the format: "hours:minutes:seconds".
 */
export function formatSecondToDuration(second: number): string {
	const hours = Math.floor(second / 3600);
	const minutes = Math.floor((second % 3600) / 60);
	const seconds = Math.floor((second % 3600) % 60);

	return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
}
