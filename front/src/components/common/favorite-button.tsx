import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { MouseEvent, useCallback, useState } from "react";
import { atomIsLogin } from "../../stores/auth";
import { useNavigate } from "react-router-dom";
import { Article } from "../../api/article";
import { errHandler } from "../../utils";
import { atomFavorTrigger, atomFeedQueryType } from "../../stores/feed";

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
  const isLogin = useAtomValue(atomIsLogin);
  const navigate = useNavigate();
  const setFavorTrigger = useSetAtom(atomFavorTrigger);
  const feedType = useAtomValue(atomFeedQueryType);

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

  const onClick = useCallback(async () => {
    if (!isLogin) {
      navigate("/login");
      return;
    }
    setLoading(true);
    try {
      await (favorited ? disFavorite() : favorite());
    } catch (error) {
      console.error('Error toggling favorite:', error);
    }
    setLoading(false);

    if(feedType == "user") {
      console.log("changed");
      setFavorTrigger(true);
    }
  }, [isLogin, navigate, favorited]);

  return (
    <button
      type="button"
      className={`btn btn-sm ${favorited ? 'btn-primary' : 'btn-outline-primary'} ${loading && 'disabled'}`}
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
