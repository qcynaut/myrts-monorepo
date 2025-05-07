/**
 * Represents an error returned by the API.
 */
export type ApiError = {
	error: string; // The error message.
};

/**
 * Represents a message returned by the API.
 */
export type Message = {
	message: string; // The message content.
};

/**
 * Represents the response from a verification request.
 */
export type VerifyRes = {
	token: string; // The verification token.
	mobile: boolean; // Indicates if the verification is for a mobile device.
};

/**
 * Represents the response from an authentication request.
 */
export type AuthRes = {
	token: string; // The authentication token.
	pending: boolean; // Indicates if the authentication is pending.
};

/**
 * Represents an object with an ID and a name.
 */
export type IdName = {
	id: number; // ID of the object
	name: string; // Name of the object
};

/**
 * Represents city data, including an ID, name, and province information.
 */
export type CityData = {
	id: number; // ID of the city
	name: string; // Name of the city
	province: IdName; // Province information of the city
};

/**
 * Represents a city.
 */
export type City = {
	id: number; // ID of the city
	name: string; // Name of the city
	province_id: number; // ID of the province the city belongs to
};

/**
 * Represents a package, including an ID, name, and maximum number of devices.
 */
export type Package = {
	id: number; // ID of the package
	name: string; // Name of the package
	max_devices: number; // Maximum number of devices allowed for the package
};

/**
 * Represents a group, including a description, ID, name, and parent ID.
 */
export type Group = {
	description: string | null; // Description of the group (nullable)
	id: number; // ID of the group
	name: string; // Name of the group
	parent_id: number | null; // ID of the parent group (nullable)
};
/**
 * Represents a subscription, including an expiration date, order date, package information, and user ID.
 */
export type Subscription = {
	expire_date: string; // Expiration date of the subscription
	id: number; // ID of the subscription
	order_date: string; // Order date of the subscription
	package: Package; // Package information of the subscription
	user_id: number; // ID of the user associated with the subscription
};

/**
 * Represents a user, including city ID, device IDs, email, ID, image URL, name, role ID, and user group IDs.
 */
export type User = {
	city_id?: number | null; // City ID of the user
	city?: CityData | null; // City data of the user
	device_ids: number[]; // Device IDs associated with the user
	email: string; // Email of the user
	id: number; // ID of the user
	image_url: string | null; // Image URL of the user (nullable)
	name: string; // Name of the user
	role_id?: number; // Role ID of the user
	role?: IdName; // Role information of the user
	user_group_ids: number[]; // User group IDs associated with the user
	subscription?: Subscription | null; // Subscription information of the user
};

/**
 * Represents the response object for statistics, including avs, duration, records, schedule, and unit.
 */
export type StatisticsRes = {
	avs: number; // AVS value
	duration: number; // Duration value
	records: number; // Records value
	schedule: number; // Schedule value
	unit: number; // Unit value
};

/**
 * Represents the result object with a generic type, including the result, error (optional), and status.
 */
export interface Result<T> {
	result: T | null; // Result value
	error?: ApiError; // Error object (optional)
	status: number; // Status code
}

/**
 * Represents a paginated list of items.
 */
export type Paginated<T> = {
	current_page: number; // The current page number.
	items: T[]; // The list of items.
	next: string | null; // The URL of the next page, if available.
	prev: string | null; // The URL of the previous page, if available.
	total: number; // The total number of items.
	total_page: number; // The total number of pages.
};

/**
 * Represents a group of data.
 */
export type GroupData = {
	children: GroupData[]; // The list of child groups.
	description: string | null; // The description of the group, if available.
	id: number; // The unique identifier of the group.
	name: string; // The name of the group.
	parent_id: number | null; // The ID of the parent group, if any.
	users: User[]; // The list of users in the group.
};

/**
 * Represents an AVS (Address Validation Service).
 */
export type Avs = {
	address: string | null; // The address associated with the AVS.
	description: string | null; // The description of the AVS.
	cpu_temp: string | null; // The CPU temperature of the AVS.
	disk_free: string | null; // The free disk space of the AVS.
	disk_total: string | null; // The total disk space of the AVS.
	id: number; // The unique identifier of the AVS.
	kind: number; // The kind of the AVS.
	lat: number | null; // The latitude of the AVS.
	lng: number | null; // The longitude of the AVS.
	mem_free: string | null; // The free memory of the AVS.
	mem_total: string | null; // The total memory of the AVS.
	networks: string | null; // The networks associated with the AVS.
	pending: number; // The pending indicator of the AVS.
	status: number; // The status of the AVS.
	unique_id: string; // The unique identifier of the AVS.
};

/**
 * Represents a record.
 */
/**
 * Represents a record in the system.
 */
export type Records = {
	created_at: string; // The date and time the record was created.
	description: string | null; // The description of the record.
	duration: string; // The duration of the record.
	file_url: string; // The URL of the file associated with the record.
	hash: string; // The hash of the record.
	id: number; // The unique identifier of the record.
	name: string; // The name of the record.
	status: number; // The status of the record.
	user_id: number; // The ID of the user associated with the record.
	sender: User | null; // The sender of the record.
};

/**
 * Represents a schedule in the system.
 */
export type Schedule = {
	id: number; // The unique identifier of the schedule.
	name: string; // The name of the schedule.
	kind: number; // The kind of the schedule.
	dates: number[]; // The dates associated with the schedule.
	days: number[]; // The days associated with the schedule.
	device_ids: number[]; // The device IDs associated with the schedule.
	month: number | null; // The month associated with the schedule (or null if not applicable).
	records_id: number; // The ID of the record associated with the schedule.
	times: string[]; // The times associated with the schedule.
	user_id: number; // The ID of the user associated with the schedule.
	volumes: string[]; // The volumes associated with the schedule.
	weeks: number[]; // The weeks associated with the schedule.
	year: number | null; // The year associated with the schedule (or null if not applicable).
};
