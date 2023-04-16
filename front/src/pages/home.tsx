import { Helmet, HelmetProvider } from "react-helmet-async";
import { Feed } from "../components/common/feed";
import { FeedToggle } from "../components/home/feed-toggle";
import { PopTagsBar } from "../components/home/pop-tags-bar";
import { useEffect } from "react";
import { useAtom } from "jotai";
import { atomQueryLimit } from "../stores/feed";

export function Home() {
  // ANCHOR store
  const [, setLimit] = useAtom(atomQueryLimit);

  // ANCHOR initialize
  useEffect(() => {
    setLimit(10);
  }, [])

  return (
    <>
      <HelmetProvider>
        <Helmet>
          <title>Home - Conduit</title>
        </Helmet>
      </HelmetProvider>
      <div className="home-page">
        <div className="banner">
          <div className="container">
            <h1 className="logo-font">conduit</h1>
            <p>A place to share your knowledge.</p>
          </div>
        </div>

        <div className="container page">
          <div className="row">
            <div className="col-md-9">
              <FeedToggle />
              <Feed />
            </div>

            <div className="col-md-3">
              <PopTagsBar />
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
