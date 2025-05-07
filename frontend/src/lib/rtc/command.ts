import StreamMessage from './message';
import type { Authenticate, CommandReq, CommandRes } from './types';

class Command {
	private readonly stream: WebSocket;
	private readonly streamHandler: Record<string, (data: string) => void> = {};
	public response_handler: (response: string) => void = () => {};

	/**
	 * Initializes a new instance of the constructor.
	 *
	 * @param {WebSocket} stream - The WebSocket stream.
	 */
	constructor(stream: WebSocket) {
		this.stream = stream;
		this.streamHandler['pong'] = async () => {
			// sleep for 5 seconds
			await new Promise((resolve) => setTimeout(resolve, 5000));
			this.heartbeat();
		};
		this.handle();
		this.heartbeat();
	}

	/**
	 * Creates a new instance of Command by establishing a WebSocket connection to the specified URL.
	 *
	 * @param {string} url - The URL to establish the WebSocket connection.
	 * @return {Promise<Command>} A Promise that resolves to a new instance of Command once the WebSocket connection is successfully established.
	 */
	public static create(url: string): Promise<Command> {
		return new Promise((resolve, reject) => {
			const ws = new WebSocket(url);
			ws.onopen = () => {
				resolve(new Command(ws));
			};
			ws.onerror = (error) => {
				reject(error);
			};
		});
	}

	/**
	 * Sends a heartbeat message to the stream.
	 */
	private heartbeat() {
		this.send('ping');
	}

	/**
	 * Sends an event with optional data.
	 *
	 * @param {string} event - The name of the event to send.
	 * @param {string} [data] - Optional data to send with the event.
	 */
	private send(event: string, data?: string) {
		const message = new StreamMessage(event, data || '');
		this.stream.send(message.toJson());
	}

	private handle() {
		this.streamHandler['command'] = async (data) => {
			const response: CommandRes = JSON.parse(data);
			this.response_handler(response.response);
		};
		this.stream.onmessage = (event) => {
			const message = StreamMessage.fromJson(event.data);
			if (this.streamHandler[message.event]) {
				this.streamHandler[message.event](message.data);
			}
		};
	}

	public start(token: string) {
		const auth: Authenticate = {
			client_id: token,
			client_address: '',
			client_description: '',
			client_type: 1
		};
		this.send('auth', JSON.stringify(auth));
	}

	public command(command: string, target: string, user_id: number) {
		const req: CommandReq = {
			sender: user_id,
			command: command,
			target: target
		};
		this.send('command', JSON.stringify(req));
	}
}

export default Command;
