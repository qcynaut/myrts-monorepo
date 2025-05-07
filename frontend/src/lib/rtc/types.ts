/**
 * Represents the authentication details.
 */
export type Authenticate = {
	client_id: string; // The client ID.
	client_type: number; // The type of client.
	client_description: string; // The description of the client.
	client_address: string; // The address of the client.
};

/**
 * Represents the turn details.
 */
export type Turn = {
	url: string; // The URL for turn.
	username: string; // The username for turn.
	password: string; // The password for turn.
};

/**
 * Represents an offer.
 */
export type Offer = {
	offer: string; // The offer string.
	target: string[]; // The target array.
};

/**
 * Represents the ICE servers.
 */
export type Ices = {
	ices: string; // The ICE servers string.
};

/**
 * Represents an answer.
 */
export type Answer = {
	answer: string; // The answer string.
};

/**
 * Represents a volume.
 */
export type Volume = {
	volume: string; // The volume value
};

/**
 * Represents a command request.
 */
export type CommandReq = {
	sender: number; // The sender ID
	command: string; // The command to be executed
	target: string; // The target of the command
};

/**
 * Represents a command response.
 */
export type CommandRes = {
	sender: number; // The sender ID
	target: string; // The target of the command
	response: string; // The result of the command
};
