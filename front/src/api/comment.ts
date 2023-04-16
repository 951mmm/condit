import { DELETE, GET, POST } from "./config";
import { Profile } from "./profile";

export namespace Comment {
  export interface Comment {
    id: number;
    createdAt: string;
    body: string;
    author: Profile.Profile;
  }
  export namespace list {
    export function handler(slug: string): Promise<{
      comments: Comment[];
    }> {
      return GET(`/articles/${slug}/comments`);
    }
  }

  export namespace post {
    interface CommentReq extends Pick<Comment, "body"> {}
    export function handler(
      slug: string,
      body: {
        comment: CommentReq;
      }
    ): Promise<{ comment: Comment }> {
      return POST(`/articles/${slug}/comments`, body);
    }
  }

  export namespace del {
    export function handler(slug: string, id: number) {
      return DELETE(`/articles/${slug}/comments/${id}`);
    }
  }
}
