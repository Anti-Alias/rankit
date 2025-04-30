import { useState } from "react";
import { Category, fetchCategories } from "../../utils/category";
import { Page } from "../../utils/page";
import "./Categories.css";
import { SearchInput } from "../widgets/SearchInput";


export function Categories() {
  const [categories, setCategories] = useState<Page<Category>>({ data: [] });
  const [search, setSearch] = useState("");
  const searchCategories = async (search: string) => {
    setCategories(await fetchCategories(search));
  };
  const categoryCards = categories.data.map(category => {
    const src = `images/categories/${category.image}`;
    return (
      <li key={category.id}>
        <button type="button" className="card">
          <img className="card-image" src={src} />
          {category.name}
        </button>
      </li>
    )
  });
  return (
    <div className="Categories">
      <SearchInput search={searchCategories} placeholder="Search Categories" />
      <ul className="card-list">{categoryCards}</ul>
      {
        categories.cursor &&
        <button className="primary">Load More</button>
      }
    </div>
  )
}


