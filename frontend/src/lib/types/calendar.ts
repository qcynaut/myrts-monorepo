// Represents the possible views of the calendar
export type CalendarView = 'timeGridWeek' | 'dayGridMonth';

// Represents the parameters for setting the start and end dates
export type DateSetParams = {
	start: Date;
	end: Date;
};

// Represents the properties of a calendar event
export type CalendarEvents = {
	id: string; // The unique identifier of the event
	title: string; // The title of the event
	start: Date; // The start date of the event
	end: Date; // The end date of the event
	display: 'auto' | 'background'; // The display mode of the event
};

// Represents the options for configuring the calendar
export type CalendarOptions = {
	view: CalendarView; // The initial view of the calendar
	allDaySlot: boolean; // Whether to display an all-day slot
	dayHeaderFormat: any; // The format of the day headers
	locale: string; // The locale of the calendar
	headerToolbar: any; // The toolbar configuration
	slotDuration: string; // The duration of each slot
	datesSet: (params: DateSetParams) => void; // The callback function for date set events
	events: CalendarEvents[]; // The array of events to display on the calendar
	dayMaxEvents: boolean; // Limit the maximum number of events to display on a day
	displayEventEnd: boolean; // Whether to display the end time of events
	eventClassNames: (info: { event: CalendarEvents; view: CalendarView }) => string[]; // The class names for events
	eventClick: (info: { event: CalendarEvents }) => void; // The callback function for event click
};
