import { useAtom, useAtomValue } from "jotai";
import { atomFeedQuery, atomFeedQueryType, atomPage, atomQueryLimit } from "../../stores/feed";
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

  // ANCHOR effect
  useEffect(() => {
    setPage(1);
  }, [feedType, feedQuery]);

  useEffect(() => {
    const controller = new AbortController();
    const  { signal } = controller;
    

    async function initArticles() {
      setLoading(true);
      try {
        const { articles, articlesCount } = await Article.list.handler(
          `${feedQuery}limit=${limit}&offset=${limit * (page - 1)}`,
          signal
        );
        console.log(articles);
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
  }, [feedType, feedQuery, page]);

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
  )

  
}
