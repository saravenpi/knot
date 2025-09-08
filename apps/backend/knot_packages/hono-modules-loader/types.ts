import type { Context, Env } from "hono";

export interface AppEnv extends Env {
  Variables: {
    userId: number;
    "user-google"?: any;
    jwtPayload?: { userId?: number };
    token?: string;
    "refresh-token"?: string;
    "granted-scopes"?: string[];
  };
  Bindings: {};
  Validated: {
    json: any;
  };
}

export type AppContext = Context<{
  Variables: {
    userId: number;
  };
  Validated: {
    json: any;
  };
}>;

export interface WebSocketMessage {
  type: string;
  payload: any;
}
