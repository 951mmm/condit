import { useAtom } from "jotai";
import { MouseEvent, useState } from "react";
import { atomIsLogin } from "../../stores/auth";
import { useNavigate } from "react-router-dom";
import { Article } from "../../api/article";
import { errHandler } from "../../utils";

interface FavoriteButtonProps {
  slug: string;
  favoritesCount: number;
  favorited: boolean;
  type: "simple" | "word";
}

export function FavoriteButton({
  slug,
  favoritesCount: propFavoritesCount,
  favorited: propFavorited,
  type,
}: FavoriteButtonProps) {
  // ANCHOR state
  const [favorited, setFavorited] = useState(propFavorited);
  const [favoritesCnt, setFavoritesCnt] = useState(propFavoritesCount);
  const [loading, setLoading] = useState(false);

  // ANCHOR store
  const [isLogin] = useAtom(atomIsLogin);
  const navigate = useNavigate();

  // ANCHOR event
  async function favorite() {
    try {
      await Article.favorite.handler(slug);
      setFavorited(true);
      setFavoritesCnt((favoritesCnt) => favoritesCnt + 1);
    } catch (e) {
      errHandler(e);
    }
  }

  async function disFavorite() {
    try {
      await Article.disFavorite.handler(slug);
      setFavorited(false);
      setFavoritesCnt((favoritesCnt) => favoritesCnt - 1);
    } catch (e) {
      errHandler(e);
    }
  }

  async function onClick() {
    if (!isLogin) {
      navigate("/login");
      return;
    }
    setLoading(true);
    favorited ? await disFavorite() : await favorite();
    setLoading(false);
  }

  return (
    <button
      type="button"
      className={
        favorited
          ? `btn btn-sm btn-primary ${loading ? "disabled" : ""}`
          : `btn btn-sm btn-outline-primary ${loading ? "disabled" : ""}`
      }
      onClick={onClick}
    >
      {
        {
          simple: (
            <>
              <i className="ion-heart" /> {favoritesCnt}
            </>
          ),
          word: (
            <>
              <i className="ion-heart" />
              {` 
                ${favorited ? "Unfavorite" : "Favorite"} Post`}
              <span className="counter">({favoritesCnt})</span>
            </>
          ),
        }[type]
      }
    </button>
  );
}
