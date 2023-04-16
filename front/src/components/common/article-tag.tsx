interface ArticleTagProps {
  name: string;
}

export function ArticleTag({ name }: ArticleTagProps) {
  return <li className="tag-default tag-pill tag-outline">{` ${name} `}</li>;
}
