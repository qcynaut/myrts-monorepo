import type { NavlinkType } from '$lib/types/navigation';
import { Icon } from './icon';

export const DASHBOARD = '/dashboard';
export const DASHBOARD_PROFILE = '/dashboard/profile';
export const DASHBOARD_REPORT = '/dashboard/report';
export const DASHBOARD_STREAMING_START = '/dashboard/streaming/start';
export const DASHBOARD_STREAMING_ONGOING = '/dashboard/streaming/ongoing';
export const DASHBOARD_STREAMING_WAITING = '/dashboard/streaming/waiting';
export const DASHBOARD_STREAMING_REPORT = '/dashboard/streaming/report';
export const DASHBOARD_RECORD_ADD = '/dashboard/record/add';
export const DASHBOARD_RECORD_WAITING = '/dashboard/record/waiting';
export const DASHBOARD_RECORD_LIST = '/dashboard/record/list';
export const DASHBOARD_RECORD_REPORT = '/dashboard/record/report';
export const DASHBOARD_SCHEDULE = '/dashboard/schedule';
export const DASHBOARD_SCHEDULE_ADD = '/dashboard/schedule/add';
export const DASHBOARD_AVS = '/dashboard/avs';
export const DASHBOARD_USER = '/dashboard/user';
export const DASHBOARD_USER_ADD = '/dashboard/user/add';
export const CONTACT = 'mailto:support@myrts.id';

/**
 * Array of navigation links.
 * This constant is used in the `/src/lib/components/layout/Navbar.svelte` and `/src/lib/components/layout/Sidebar.svelte` components.
 */
export const navigations: NavlinkType[] = [
	{
		id: 1,
		group: 2,
		kind: 'link',
		link: DASHBOARD_PROFILE,
		name: 'Profile',
		user: true
	},
	{
		id: 2,
		group: 2,
		kind: 'link',
		link: DASHBOARD,
		name: 'Dashboard',
		icon: Icon.BorderAll,
		user: true
	},
	// {
	// 	id: 3,
	// 	group: 2,
	// 	kind: 'link',
	// 	link: DASHBOARD_REPORT,
	// 	name: 'Report (Analisis)',
	// 	icon: Icon.ExclamationCircle,
	// 	user: true
	// },
	{
		id: 4,
		group: 3,
		kind: 'dropdown',
		name: 'Streaming',
		icon: Icon.Podcast,
		children: [
			{
				id: 1,
				group: 1,
				kind: 'link',
				link: DASHBOARD_STREAMING_START,
				name: 'Mulai Streaming',
				icon: Icon.Plus,
				user: true
			},
			{
				id: 2,
				group: 1,
				kind: 'link',
				link: DASHBOARD_STREAMING_ONGOING,
				name: 'Ongoing',
				icon: Icon.Rocket,
				user: true
			},
			{
				id: 3,
				group: 1,
				kind: 'link',
				link: DASHBOARD_STREAMING_WAITING,
				name: 'Waiting List',
				icon: Icon.Tasks,
				user: true
			}
			// {
			// 	id: 4,
			// 	group: 1,
			// 	kind: 'link',
			// 	link: DASHBOARD_STREAMING_REPORT,
			// 	name: 'Report',
			// 	icon: Icon.ExclamationCircle,
			// 	user: true
			// }
		],
		user: true
	},
	{
		id: 5,
		group: 3,
		kind: 'dropdown',
		name: 'Rekaman',
		icon: Icon.Music,
		children: [
			{
				id: 1,
				group: 1,
				kind: 'link',
				link: DASHBOARD_RECORD_ADD,
				name: 'Tambah Rekaman',
				icon: Icon.Plus,
				user: true
			},
			{
				id: 2,
				group: 1,
				kind: 'link',
				link: DASHBOARD_RECORD_WAITING,
				name: 'Waiting List',
				icon: Icon.Rocket,
				user: true
			},
			{
				id: 3,
				group: 1,
				kind: 'link',
				link: DASHBOARD_RECORD_LIST,
				name: 'List Rekaman',
				icon: Icon.Tasks,
				user: true
			}
			// {
			// 	id: 4,
			// 	group: 1,
			// 	kind: 'link',
			// 	link: DASHBOARD_RECORD_REPORT,
			// 	name: 'Report',
			// 	icon: Icon.ExclamationCircle,
			// 	user: true
			// }
		],
		user: true
	},
	{
		id: 6,
		group: 3,
		kind: 'link',
		link: DASHBOARD_SCHEDULE,
		name: 'Jadwal',
		icon: Icon.Calendar,
		user: true
	},
	{
		id: 6,
		group: 3,
		kind: 'dropdown',
		name: 'User',
		icon: Icon.User,
		children: [
			{
				id: 1,
				group: 1,
				kind: 'link',
				link: DASHBOARD_USER_ADD,
				name: 'Tambah User',
				icon: Icon.UserPlus,
				user: false
			},
			{
				id: 2,
				group: 1,
				kind: 'link',
				link: DASHBOARD_USER,
				name: 'List User',
				icon: Icon.User,
				user: true
			}
		],
		user: true
	},
	{
		id: 7,
		group: 3,
		kind: 'link',
		name: 'AVS',
		icon: Icon.Server,
		link: DASHBOARD_AVS,
		user: true
	},
	{
		id: 9,
		group: 4,
		kind: 'link',
		name: 'Contact',
		icon: Icon.Phone,
		link: CONTACT,
		user: true
	}
];
