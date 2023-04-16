import { useAtom, useAtomValue } from "jotai";
import { atomPage, atomPageLink, atomQueryLimit } from "../../stores/feed";
import { Link } from "react-router-dom";

interface PaginationProps {
  articlesCount: number;
}

export function Pagination({ articlesCount }: PaginationProps) {
  // ANCHOR store
  const [page, setPage] = useAtom(atomPage);
  const limit = useAtomValue(atomQueryLimit);
  const pageLink = useAtomValue(atomPageLink);
  const pageCnt = Math.ceil(articlesCount / limit);
  const addOne = (n: number) => n + 1;
  const pageIndex = [...Array(pageCnt).keys()].map(addOne);

  if (pageCnt === 1) return <></>;
  return (
    <nav>
      <ul className="pagination">
        {pageIndex.map((index) => (
          <li
            key={index}
            className={`page-item ${page === index ? "active" : ""}`}
          >
            <Link to={pageLink} className="page-link" onClick={() => setPage(index)}>
              {index}
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}
