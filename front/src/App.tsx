import { useEffect, useState } from "react";
import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import { Home } from "./pages/home";
import { Article } from "./pages/article";
import { Header } from "./components/header";
import { Footer } from "./components/footer";
import { useAtom, useSetAtom } from "jotai";
import { atomIsLogin, atomUser } from "./stores/auth";
import { User } from "./api/user";
import { Settings } from "./pages/settings";
import { Auth } from "./pages/auth";
import { Profile } from "./pages/profile";
import { Editor } from "./pages/editor";

function App() {
  // ANCHOR state
  const [loading, setLoading] = useState(false);

  // ANCHOR store
  const setIsLogin = useSetAtom(atomIsLogin);
  const setUser = useSetAtom(atomUser);

  // ANCHOR initialize
  useEffect(() => {
    async function initApp() {
      setLoading(true);
      const hasToken = !!localStorage.getItem("jwtToken");
      if (!hasToken) return;
      try {
        const data = await User.get.handler();
        const { token, ...rest } = data.user;
        setIsLogin(true);
        setUser(rest);
      } catch (e) {
        localStorage.removeItem("jwtToken");
        setIsLogin(false);
        setUser({
          email: "",
          username: "",
          bio: "",
          image: "",
        });
      }
    }

    initApp().then(() => setLoading(false));
  }, []);

  // ANCHOR render
  if (loading) return <p>loading...</p>;

  return (
    <BrowserRouter>
      <Header />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Auth type="login" />} />
        <Route path="/register" element={<Auth type="register" />} />
        <Route path="/article/:URLSlug" element={<Article />} />
        <Route path="/editor" element={<Editor type="new" />} />
        <Route path="/editor/:URLSlug" element={<Editor type="update" />} />
        <Route path="/settings" element={<Settings />} />
        <Route path="/profile/:userId/*" element={<Profile />} />
        <Route path="*" element={<Navigate to="/" replace={true} />} />
      </Routes>
      <Footer />
    </BrowserRouter>
  );
}

export default App;
