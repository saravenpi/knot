import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import type { Hono } from "hono";
import type { AppEnv } from "./types";

/**
 * Dynamically loads all route modules from the routes directory
 * @param {Hono} webserver - The Hono application instance
 * @returns {Promise<void>} Loads and registers all route handlers
 */
export async function loadModules(
  webserver: Hono | Hono<AppEnv>,
  modulesPath?: string,
): Promise<void> {
  const routesDir = modulesPath || "src/modules";
  for (const entry of readdirSync(routesDir)) {
    const moduleDir = join(routesDir, entry);
    if (!statSync(moduleDir).isDirectory()) continue;

    const routeFile = join(moduleDir, `router.ts`);
    try {
      const { default: routeHandler } = await import(
        pathToFileURL(routeFile).href
      );

      webserver.route(`/${entry}`, routeHandler);
      console.info(`[modules] Loaded module "${entry}" from ${routeFile}`);
    } catch (err: any) {
      if (
        err.code === "ERR_MODULE_NOT_FOUND" ||
        err.code === "MODULE_NOT_FOUND"
      ) {
        console.warn(`[modules] (skip) No route file for module "${entry}"`);
        console.warn("tried to access but error: ", err.message);
        continue;
      }
      throw err;
    }
  }
  console.info("Modules loaded successfully");
}

export default loadModules;
