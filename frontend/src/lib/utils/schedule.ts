import type { CalendarEvents } from '$lib/types/calendar';
import type { Records, Schedule } from '$lib/types/response';

/**
 * Retrieves the schedules for the current day.
 *
 * @param {Schedule[]} schedules - An array of Schedule objects.
 * @return {Schedule[]} - An array of Schedule objects representing the schedules for the current day.
 */
export function getTodaySchedule(schedules: Schedule[]): Schedule[] {
	const res: Schedule[] = [];

	const year = new Date().getFullYear();
	const month = new Date().getMonth();
	const date = new Date().getDate();
	const week_number = getWeekNumber(new Date(year, month, date));
	const day_number = new Date(year, month, date).getDay();

	for (const schedule of schedules) {
		if (schedule.kind === 1) {
			if (schedule.weeks.includes(week_number) && schedule.days.includes(day_number)) {
				res.push(schedule);
			}
			if (schedule.dates.includes(date)) {
				res.push(schedule);
			}
		} else {
			if (schedule.year === year && schedule.month === month && schedule.dates.includes(date)) {
				res.push(schedule);
			}
		}
	}

	return res;
}

/**
 * Calculates the week number of a given date.
 *
 * @param {Date} date - The date for which to calculate the week number.
 * @return {number} The week number of the given date.
 */
function getWeekNumber(date: Date): number {
	const adjusted = date.getDate() + date.getDay();
	const prefixses = [0, 1, 2, 3, 4, 5];
	return prefixses[0 | (adjusted / 7)] + 1;
}

function getDateForWeekAndDay(year: number, month: number, week: number, day: number): Date | null {
	const date = new Date(year, month - 1, 1); // Subtract 1 from the month since it's zero-based
	const firstDayOfWeek = date.getDay(); // Get the day of the week for the first day of the month

	// Calculate the offset to the desired week and day
	const daysOffset = (week - 1) * 7 + (day - 1 - firstDayOfWeek);

	// Adjust the date by the offset
	date.setDate(date.getDate() + daysOffset);

	// Check if the adjusted date is still within the same month
	if (date.getMonth() !== month - 1) {
		return null; // Return null if the date is in a different month
	}

	return date;
}
function getMonthFromRange(
	startMonth: number,
	endMonth: number
): { month: number; nextYear: boolean }[] {
	const months: { month: number; nextYear: boolean }[] = [];

	if (startMonth <= endMonth) {
		for (let i = startMonth; i <= endMonth; i++) {
			months.push({ month: i, nextYear: false });
		}
	} else {
		for (let i = startMonth; i <= 12; i++) {
			months.push({ month: i, nextYear: false });
		}
		for (let i = 1; i <= endMonth; i++) {
			months.push({ month: i, nextYear: true });
		}
	}
	return months;
}

function expandScheduleRepetition(
	start: Date,
	end: Date,
	schedule: Schedule,
	duration: number
): CalendarEvents[] {
	const events: CalendarEvents[] = [];

	const startMonth = start.getMonth() + 1;
	const endMonth = end.getMonth() + 1;

	if (startMonth === endMonth) {
		// week: 1-5
		for (const week of schedule.weeks) {
			// day: 1-7
			for (const day of schedule.days) {
				for (const time of schedule.times) {
					const date = getDateForWeekAndDay(start.getFullYear(), startMonth, week, day);
					if (!date) continue;
					date.setHours(parseInt(time.split(':')[0]) || 0);
					date.setMinutes(parseInt(time.split(':')[1]) || 0);
					events.push({
						start: date,
						end: new Date(date.getTime() + duration),
						title: schedule.name,
						display: 'auto',
						id: `${schedule.id}_rep_${week}_${day}_${time}`
					});
				}
			}
		}

		for (const d of schedule.dates) {
			for (const time of schedule.times) {
				const date = new Date();
				date.setFullYear(start.getFullYear());
				date.setMonth(startMonth - 1);
				date.setDate(d);
				date.setHours(parseInt(time.split(':')[0]) || 0);
				date.setMinutes(parseInt(time.split(':')[1]) || 0);
				events.push({
					start: date,
					end: new Date(date.getTime() + duration),
					title: schedule.name,
					display: 'auto',
					id: `${schedule.id}_rep_${d}_${time}`
				});
			}
		}
	} else {
		for (const week of schedule.weeks) {
			for (const day of schedule.days) {
				for (const time of schedule.times) {
					const months = getMonthFromRange(startMonth, endMonth);
					for (const month of months) {
						const year = month.nextYear ? start.getFullYear() + 1 : start.getFullYear();
						const date = getDateForWeekAndDay(year, month.month, week, day);
						if (!date) continue;
						date.setHours(parseInt(time.split(':')[0]) || 0);
						date.setMinutes(parseInt(time.split(':')[1]) || 0);
						events.push({
							start: date,
							end: new Date(date.getTime() + duration),
							title: schedule.name,
							display: 'auto',
							id: `${schedule.id}_rep_${week}_${day}_${time}_${year}_${month.month}`
						});
					}
				}
			}
		}

		for (const d of schedule.dates) {
			for (const time of schedule.times) {
				const months = getMonthFromRange(startMonth, endMonth);
				for (const month of months) {
					const year = month.nextYear ? start.getFullYear() + 1 : start.getFullYear();
					const date = new Date();
					date.setFullYear(year);
					date.setMonth(month.month - 1);
					date.setDate(d);
					date.setHours(parseInt(time.split(':')[0]) || 0);
					date.setMinutes(parseInt(time.split(':')[1]) || 0);
					events.push({
						start: date,
						end: new Date(date.getTime() + duration),
						title: schedule.name,
						display: 'auto',
						id: `${schedule.id}_rep_${d}_${time}_${year}_${month.month}`
					});
				}
			}
		}
	}

	return events;
}

export function expandSchedule(
	start: Date,
	end: Date,
	schedules: Schedule[],
	records: Records[]
): CalendarEvents[] {
	const events: CalendarEvents[] = [];

	for (const schedule of schedules) {
		const duration: number =
			parseInt(records.find((r) => r.id === schedule.records_id)?.duration || '0') * 1000;
		if (schedule.kind === 2) {
			const date = new Date();
			date.setFullYear(schedule.year || 0);
			date.setMonth((schedule.month || 1) - 1);
			date.setDate(1);
			for (const d of schedule.dates) {
				date.setDate(d);
				for (const time of schedule.times) {
					date.setHours(parseInt(time.split(':')[0]) || 0);
					date.setMinutes(parseInt(time.split(':')[1]) || 0);
					events.push({
						start: date,
						end: new Date(date.getTime() + duration),
						title: schedule.name,
						display: 'auto',
						id: `${schedule.id}_${d}_${time}`
					});
				}
			}
		} else {
			events.push(...expandScheduleRepetition(start, end, schedule, duration));
		}
	}

	return events;
}
