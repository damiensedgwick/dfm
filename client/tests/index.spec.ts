import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
	await page.goto('http://localhost:5173/');
});

const TODO_ITEMS = [
	{
		title: 'win a game of warzone 2.0',
		completed: false,
		archived: false
	},
	{
		title: 'buy some cheese',
		completed: false,
		archived: false
	},
	{
		title: 'actually complete a side project',
		completed: false,
		archived: false
	},
];

test.describe('Create a new Todo', () => {
	test('should allow me to add todo items', async ({ page }) => {
		// create a new todo locator.
		const newTodo = page.locator('data-test-id=new-todo');

		// create 1st todo.
		await newTodo.fill(TODO_ITEMS[0].title);
		await newTodo.press('Enter');

		// make sure the list only has one todo item.
		await expect(page.getByTestId('todo-title')).toHaveText([TODO_ITEMS[0].title]);

		// // create 2nd todo.
		// await newTodo.fill(TODO_ITEMS[1].title);
		// await newTodo.press('Enter');
		//
		// // make sure the list has two todo items.
		// await expect(page.getByTestId('todo-title')).toHaveText([TODO_ITEMS[0].title, TODO_ITEMS[1].title]);
	});
});