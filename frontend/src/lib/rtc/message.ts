/**
 * @class StreamMessage
 * Represents an event and its data.
 */
class StreamMessage {
	// Represents an event
	public readonly event: string;

	// Represents the data
	public readonly data: string;

	/**
	 * Creates a new instance of the constructor.
	 *
	 * @param {string} event - The event to be passed to the constructor.
	 * @param {string} data - The data to be passed to the constructor.
	 */
	constructor(event: string, data: string) {
		this.event = event;
		this.data = data;
	}

	/**
	 * Converts the current object to a JSON string.
	 *
	 * @return {string} The JSON representation of the object.
	 */
	public toJson(): string {
		return JSON.stringify(this);
	}

	/**
	 * Parses a JSON string and returns a StreamMessage object.
	 *
	 * @param {string} json - The JSON string to parse.
	 * @return {StreamMessage} The parsed StreamMessage object.
	 */
	public static fromJson(json: string): StreamMessage {
		return JSON.parse(json);
	}
}

export default StreamMessage;
