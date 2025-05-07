import type { Icon } from '$lib/const/icon';

/**
 * Type definition for the Navigation.
 * @property {number} id - The unique identifier of the navigation link.
 * @property {number} group - The group number that the navigation link belongs to.
 * @property {'dropdown' | 'link'} kind - The type of the navigation link. It can either be a 'dropdown' or a 'link'.
 * @property {string} name - The display name of the navigation link.
 * @property {string} [link] - The URL that the navigation link points to. This property is optional.
 * @property {Icon} [icon] - The icon of the navigation link. This property is optional.
 * @property {NavlinkType[]} [children] - The child navigation links of the current navigation link. This property is optional.
 */
export type NavlinkType = {
	id: number;
	group: number;
	kind: 'dropdown' | 'link';
	name: string;
	link?: string;
	icon?: Icon;
	children?: NavlinkType[];
	user: boolean;
};
