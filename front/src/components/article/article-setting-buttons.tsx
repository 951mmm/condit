import { useNavigate } from "react-router-dom";
import { Article as ArticleApi } from "../../api/article";
import { errHandler } from "../../utils";

interface ArticleSettingButtonsProps {
  slug: string;
}

export function ArticleSettingButtons({ slug }: ArticleSettingButtonsProps) {
  // ANCHOR store
  const navigate = useNavigate();

  // ANCHOR event
  async function onDeleteArticle() {
    try {
      await ArticleApi.del.handler(slug);
      navigate(-1);
    } catch (e) {
      errHandler(e);
    }
  }

  return (
    <>
      <button
        className="btn btn-sm btn-outline-secondary"
        type="button"
        onClick={() => navigate(`/editor/${slug}`)}
      >
        <i className="ion-edit" />
        Edit Article
      </button>
      {" "}
      <button
        className="btn btn-sm btn-outline-danger"
        type="button"
        onClick={onDeleteArticle}
      >
        <i className="ion-trash-a" />
        Delete Article
      </button>
    </>
  );
}
