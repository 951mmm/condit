import { Link, useNavigate } from "react-router-dom";
import { Article } from "../../api/article";
import { useState } from "react";
import { ArticleTag } from "./article-tag";
import { FavoriteButton } from "./favorite-button";

interface ArticlePreviewProps {
  article: Article.Article;
}

export function ArticlePreview({ article }: ArticlePreviewProps) {
  return (
    <div className="article-preview">
      <div className="article-meta">
        <Link to={`/profile/${article.author.username}`}>
          <img
            src={article.author.image}
            alt={`avatar ${article.author.username}`}
          />
        </Link>

        <div className="info">
          <Link to={`/profile/${article.author.username}`} className="author">
            {article.author.username}
          </Link>
          <span className="date">
            {new Date(article.createdAt).toLocaleDateString("en-US", {
              month: "long",
              day: "numeric",
              year: "numeric",
            })}
          </span>
        </div>

        <div className="pull-xs-right">
          <FavoriteButton
            slug={article.slug}
            favorited={article.favorited}
            favoritesCount={article.favoritesCount}
            type="simple"
          />
        </div>
      </div>
      <Link to={`article/${article.slug}`} className="preview-link">
        <h1>{article.title}</h1>
        <p>{article.description}</p>
        <span>Read more...</span>
        <ul className="tag-list">
          {article.tagList.map((tag, index) => (
            <ArticleTag key={index} name={tag} />
          ))}
        </ul>
      </Link>
    </div>
  );
}
