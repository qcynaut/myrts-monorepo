import StreamMessage from './message';
import type { Answer, Authenticate, Ices, Offer, Turn, Volume } from './types';

/**
 * @class MyRTS
 * Represents a MyRTS instance
 */
class MyRTS {
	private readonly stream: WebSocket;
	private readonly streamHandler: Record<string, (data: string) => void> = {};
	private track: MediaStream = new MediaStream();
	private peer: RTCPeerConnection | undefined;
	private readonly ices: RTCIceCandidate[] = [];
	private unique_ids: string[] = [];
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public error_handler: (error: any) => void = () => {};
	public connected_handler: () => void = () => {};

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
	 * Creates a new instance of MyRTS by establishing a WebSocket connection to the specified URL.
	 *
	 * @param {string} url - The URL to establish the WebSocket connection.
	 * @return {Promise<MyRTS>} A Promise that resolves to a new instance of MyRTS once the WebSocket connection is successfully established.
	 */
	public static create(url: string): Promise<MyRTS> {
		return new Promise((resolve, reject) => {
			const ws = new WebSocket(url);
			ws.onopen = () => {
				resolve(new MyRTS(ws));
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

	private async begin(turn: Turn) {
		try {
			const m = await navigator.mediaDevices.getUserMedia({
				audio: {
					echoCancellation: true,
					noiseSuppression: true,
					sampleRate: 48000,
					channelCount: 2
				},
				video: false
			});
			this.peer = new RTCPeerConnection({
				iceServers: [
					{
					        urls: "turn:api.myrts.id:3478",
					        username: "myrts",
					        credential: "myrts",
					},
				]
			});
			m.getTracks().forEach((track) => {
				this.peer?.addTrack(track, m);
			});
			this.peer.onicecandidate = (event) => {
				if (event.candidate) {
					this.ices.push(event.candidate);
				}
			};
			this.peer.onicegatheringstatechange = async () => {
				if (this.peer?.iceGatheringState === 'complete') {
					while (!this.peer?.localDescription) {
						await new Promise((resolve) => setTimeout(resolve, 100));
					}
					const ices: Ices = {
						ices: JSON.stringify(this.ices)
					};
					this.send('ices', JSON.stringify(ices));
				}
			};
			this.peer.onconnectionstatechange = () => {
				if (this.peer?.connectionState === 'connected') {
					if (this.connected_handler) {
						this.connected_handler();
					}
				} else if (this.peer?.connectionState === 'failed') {
					if (this.error_handler) {
						this.error_handler('connection failed');
					}
				}
			};
			const offer = await this.peer.createOffer();
			await this.peer.setLocalDescription(offer);
			const offer_request: Offer = {
				offer: JSON.stringify(offer),
				target: this.unique_ids
			};
			this.send('offer', JSON.stringify(offer_request));
			this.track = m;
		} catch (error) {
			if (this.error_handler) {
				this.error_handler(error);
			}
		}
	}

	public getMediaStream(): MediaStream {
		return this.track;
	}

	private handle() {
		this.stream.onclose = () => {
			if (this.error_handler) {
				this.error_handler('connection closed');
			}
		};
		this.streamHandler['authenticated'] = () => {
			this.send('turn');
		};
		this.streamHandler['turn'] = (data: string) => {
			const turn: Turn = JSON.parse(data);
			this.begin(turn);
		};
		this.streamHandler['ices'] = async (data: string) => {
			const json: Ices = JSON.parse(data);
			const ices: RTCIceCandidateInit[] = JSON.parse(json.ices);
			while (!this.peer?.remoteDescription) {
				await new Promise((resolve) => setTimeout(resolve, 100));
			}
			for (const ice of ices) {
				this.peer?.addIceCandidate(ice);
			}
		};
		this.streamHandler['answer'] = (data: string) => {
			const json: Answer = JSON.parse(data);
			const answer: RTCSessionDescriptionInit = JSON.parse(json.answer);
			this.peer?.setRemoteDescription(answer);
		};
		this.stream.onmessage = (event) => {
			const message = StreamMessage.fromJson(event.data);
			if (this.streamHandler[message.event]) {
				this.streamHandler[message.event](message.data);
			}
		};
	}

	public async start(token: string, unique_ids: string[]) {
		this.unique_ids = unique_ids;
		const auth: Authenticate = {
			client_id: token,
			client_address: '',
			client_description: '',
			client_type: 1
		};
		this.send('auth', JSON.stringify(auth));
	}

	public stop() {
		this.connected_handler = () => {};
		this.error_handler = () => {};
		if (this.track) {
			this.track.getTracks().forEach((track) => {
				track.stop();
			});
		}
		if (this.peer) {
			this.peer.close();
		}
		this.stream.close();
	}

	public pause() {
		this.track.getTracks().forEach((track) => {
			track.enabled = false;
		});
	}

	public resume() {
		this.track.getTracks().forEach((track) => {
			track.enabled = true;
		});
	}

	public volume(volume: string) {
		const req: Volume = {
			volume: volume
		};
		this.send('volume', JSON.stringify(req));
	}
}

export default MyRTS;
