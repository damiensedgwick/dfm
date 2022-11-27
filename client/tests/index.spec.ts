import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
	await page.goto('http://localhost:5173/');
});

const TODO_ITEMS = [
	{
		id: 1,
		title: 'win a game of warzone 2.0',
		completed: false,
		archived: false
	},
	{
		id: 2,
		title: 'buy some cheese',
		completed: false,
		archived: false
	},
	{
		id: 3,
		title: 'actually complete a side project',
		completed: false,
		archived: false
	},
];