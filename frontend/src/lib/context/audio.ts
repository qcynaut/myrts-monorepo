import { getContext, setContext } from 'svelte';
import type { Readable, Writable } from 'svelte/store';

// Define a symbol to be used as the context key
const CONTEXT_KEY = Symbol();

// Define a type to distribute the writable properties of an object
type DistributeWritable<T> = {
	[P in keyof T]: Writable<T[P]>;
};

// Define a type to distribute the readable properties of an object
type DistributeReadable<T> = {
	[P in keyof T]: Readable<T[P]>;
};

// Define an interface for the read-only audio state
interface ReadOnlyAudioState {
	playing: boolean; // Indicates if the audio is currently playing
	ended: boolean; // Indicates if the audio has ended
	duration: number; // The duration of the audio in seconds
}

// Define an interface for the audio state
interface AudioState {
	currentTime: number; // The current playback time of the audio in seconds
	volume: number; // The volume level of the audio
	muted: boolean; // Indicates if the audio is muted
	repeat: boolean; // Indicates if the audio should repeat playback
	playbackRate: number; // The playback rate of the audio
	paused: boolean; // Indicates if the audio is paused
}

// Define a type alias for the writable audio state
export type WritableAudioState = DistributeWritable<AudioState>;

// Define a type alias for the readable audio state
export type ReadableAudioState = DistributeReadable<ReadOnlyAudioState>;

/**
 * Represents the audio context.
 * Combines the writable and readable audio states.
 */
export type AudioContext = WritableAudioState &
	ReadableAudioState & {
		/**
		 * Seeks the audio context by the specified amount.
		 * @param n - The amount to seek by.
		 */
		seekBy: (n: number) => void;

		/**
		 * Seeks the audio context to the specified time.
		 * @param t - The time to seek to.
		 */
		seekTo: (t: number) => void;
	};

/**
 * Sets the audio context.
 *
 * @param {AudioContext} context - The audio context to set.
 * @return {AudioContext} The updated audio context.
 */
export const setAudioContext = (context: AudioContext) =>
	setContext<AudioContext>(CONTEXT_KEY, context);

/**
 * Retrieves the audio context.
 *
 * @return {AudioContext} The audio context instance.
 */
export const getAudioContext = () => {
	const context = getContext<AudioContext>(CONTEXT_KEY);

	if (!context) {
		throw new Error('getAudioContext should be used in AudioPlayer`s child component');
	}

	return context;
};
