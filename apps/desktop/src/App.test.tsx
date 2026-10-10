// S0 smoke test: the browser preview must identify its missing native service clearly.
import { render, screen } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import { cleanup } from "@testing-library/react";
import App from "./App";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

test("browser preview explains that native services require the installed app", async () => {
  render(<App />);
  expect(screen.getByRole("heading", { name: "DesignWorkbench" })).toBeTruthy();
  expect(await screen.findByRole("status")).toHaveProperty(
    "textContent",
    "浏览器预览：桌面服务需在安装版中运行",
  );
});
