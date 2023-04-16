import { GET, POST, PUT } from "./config";

export namespace User {
  export interface User {
    username: string;
    email: string;
    token: string;
    bio?: string;
    image: string;
  }

  export namespace login {
    interface UserReq
      extends Omit<User, "username" | "token" | "bio" | "image"> {}
    export function handler(body: { user: UserReq }): Promise<{ user: User }> {
      return POST("/users/login", body);
    }
  }

  export namespace get {
    export function handler(): Promise<{ user: User }> {
      return GET("/user");
    }
  }

  export namespace put {
    interface UserReq extends Omit<User, "token"> {}
    export function handler(body: { user: UserReq }): Promise<{ user: User }> {
      return PUT("/user", body);
    }
  }

  export namespace post {
    export interface UserReq {
      user: {
        username: string;
        email: string;
        password: string;
      };
    }

    export function handler(body: UserReq): Promise<User> {
      return POST("/users", body);
    }
  }
}
