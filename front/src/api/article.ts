import { DELETE, GET, POST } from "./config";
import { Profile } from "./profile";
export namespace Article {
  export interface Article {
    slug: string;
    title: string;
    description: string;
    body: string;
    tagList: string[];
    createdAt: string;
    updatedAt: string;
    favorited: boolean;
    favoritesCount: 0;
    author: Profile.Profile;
  }
  export namespace list {
    export interface Articles {
      articles: Article[];
      articlesCount: number;
    }
    export function handler(
      query: string,
      signal: AbortSignal
    ): Promise<Articles> {
      return GET(`/articles${query}`, signal);
    }
  }

  export namespace get {
    export function handler(slug: string): Promise<{ article: Article }> {
      return GET(`/articles/${slug}`);
    }
  }
  export namespace favorite {
    export function handler(slug: string) {
      return POST(`/articles/${slug}/favorite`);
    }
  }

  export namespace disFavorite {
    export function handler(slug: string) {
      return DELETE(`/articles/${slug}/favorite`);
    }
  }
}
