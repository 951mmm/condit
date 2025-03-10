import { useAtom, useAtomValue } from "jotai";
import {
  atomFavorTrigger,
  atomFeedQuery,
  atomFeedQueryType,
  atomPage,
  atomQueryLimit,
  atomQueryString,
} from "../../stores/feed";
import { Article } from "../../api/article";
import { useEffect, useState } from "react";
import { ArticlePreview } from "./article-preview";
import { Pagination } from "./pagination";
import { errHandler } from "../../utils";

export function Feed() {
  // ANCHOR state
  const [loading, setLoading] = useState(false);
  const [articles, setArticles] = useState<Article.Article[]>([]);
  const [articleCount, setArticleCount] = useState(0);

  // ANCHOR store
  const feedType = useAtomValue(atomFeedQueryType);
  const feedQuery = useAtomValue(atomFeedQuery);
  const [page, setPage] = useAtom(atomPage);
  const limit = useAtomValue(atomQueryLimit);
  const [favorTrigger, setFavorTrigger] = useAtom(atomFavorTrigger);
  const queryString = useAtomValue(atomQueryString);

  // ANCHOR effect
  useEffect(() => {
    setPage(1);
  }, [feedType, feedQuery]);

  useEffect(() => {
    setFavorTrigger(false);
  }, [articleCount]);

  useEffect(() => {
    const controller = new AbortController();
    const { signal } = controller;

    async function initArticles() {
      setLoading(true);
      console.log("feed type is", feedType)
      try {
      
        const { articles, articlesCount } = await Article.list.handler(
          `${feedQuery}limit=${limit}&offset=${limit * (page - 1)}&query_string=${feedType === "tag" ? "" : queryString}`,
          signal
        );
      
        setArticles(articles);
        setArticleCount(articlesCount);
        setLoading(false);
      } catch (e) {
        errHandler(e);
      }
    }
    initArticles();

    return () => {
      controller.abort();
    };
  }, [feedType, feedQuery, page, favorTrigger, queryString]);

  // ANCHOR render
  if (loading) {
    return <div className="article-preview">loading...</div>;
  }

  if (articleCount === 0) {
    return <div className="article-preview">No articles are here... yet.</div>;
  }

  return (
    <>
      {articles.map((article, index) => (
        <ArticlePreview key={index} article={article} />
      ))}
      <Pagination articlesCount={articleCount} />
    </>
  );
}
