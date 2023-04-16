import { useAtomValue } from "jotai";
import { ReactElement } from "react";
import { atomArticle } from "../../stores/article";
import { ArticleSettingButtons } from "./article-setting-buttons";
import { FollowButton } from "../common/follow-button";
import { FavoriteButton } from "../common/favorite-button";
import { Link } from "react-router-dom";
import { atomUser } from "../../stores/auth";

export function ArticleMeta() {
  const article = useAtomValue(atomArticle);
  const user = useAtomValue(atomUser);
  console.log(user.username);
  console.log(article?.author.username)

  if (!article) return <p>loading...</p>;
  return (
    <div className="article-meta">
      <Link to={`/profile/${article?.author.username}`}>
        <img src={article?.author.image} alt={article?.author.username} />
      </Link>
      <div className="info">
        <Link to={`/profile/${article?.author.username}`} className="author">
          {article?.author.username}
        </Link>
        <span className="date">
          {new Date(article?.createdAt!).toLocaleDateString("en-US", {
            month: "long",
            day: "numeric",
            year: "numeric",
          })}
        </span>
      </div>
      {user.username === article?.author.username ? (
        <ArticleSettingButtons slug={article.slug} />
      ) : (
        <>
          <FollowButton
            following={article.author.following}
            userId={article.author.username}
          />
          <span> </span>
          <FavoriteButton
            type="word"
            slug={article.slug}
            favorited={article.favorited}
            favoritesCount={article.favoritesCount}
          />
        </>
      )}
    </div>
  );
}
