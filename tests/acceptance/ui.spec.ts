import { expect, test } from '@playwright/test';
test('preview lifecycle is explicit, usable and responsive', async ({
  page,
}) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  await page.goto('/');
  await expect(page.getByText('Preview — simulated runtime')).toBeVisible();
  await page.getByRole('button', { name: 'Start session' }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime is running.',
  );
  await page.getByRole('button', { name: 'Check connection' }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime is running.',
  );
  await page.getByRole('button', { name: 'Stop', exact: true }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime stopped.',
  );
  await page.getByRole('button', { name: 'Restart runtime' }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime is running.',
  );
  await page.getByLabel('Development profile').selectOption('student-a');
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime stopped.',
  );
  await page.screenshot({ path: '.local/ui-preview.png', fullPage: true });
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  expect(errors).toEqual([]);
});
