import { spawnSync } from "node:child_process";
import path from "node:path";
import { expect, test } from "vitest";

test("astro build outputs expected log", () => {
	const build = spawnSync("yarn", ["build"], {
		encoding: "utf8",
		stdio: "pipe",
		cwd: path.join(process.cwd(), "test"),
	});

	expect(build.stderr).toBeFalsy();
	expect(build.status).toBe(0);
});
