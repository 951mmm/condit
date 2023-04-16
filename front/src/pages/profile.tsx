import { useAtom } from "jotai";
import { Helmet, HelmetProvider } from "react-helmet-async";
import { NavLink, Route, Routes, useParams } from "react-router-dom";
import { UserInfo } from "../components/profile/user-info";
import { Feed } from "../components/common/feed";
import { ArtiflesToggle } from "../components/profile/articles-toggle";
import { useEffect } from "react";
import { atomSetFeedQuery } from "../stores/feed";

export function Profile() {
  // ANCHOR store
  const { userId } = useParams();

  return (
    <>
      <HelmetProvider>
        <Helmet>
          <title>@{userId} — Conduit</title>
        </Helmet>
      </HelmetProvider>
      <div className="profile-page">
        <UserInfo userId={userId!}/>

        <div className="container">
          <div className="row">
            <div className="col-xs-12 col-md-10 offset-md-1">
              <ArtiflesToggle userId={userId!} />
              <Routes>
                <Route path="/" element={<Feed />} />
                <Route path="/favorites" element={<Feed />} />
              </Routes>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
