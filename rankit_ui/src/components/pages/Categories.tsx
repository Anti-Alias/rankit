import { ChangeEvent, FormEvent, useEffect, useState } from "react";
import { Category, fetchCategories } from "../../utils/category";
import { Page } from "../../utils/page";
import "./Categories.css";

type State = 'NOT_LOADING' | 'LOADING' | 'ERROR';

export function Categories() {

  const [categories, setCategories] = useState<Page<Category>>({ data: [] });
  const [search, setSearch] = useState('');
  const [currentSearch, setCurrentSearch] = useState('');
  const [state, setState] = useState<State>('NOT_LOADING');

  const startSearch = async () => {
    if(state === 'LOADING') { return };
    setState('LOADING');
    setCategories({ data: [] });
    setCurrentSearch(search);
    try {
      const page = await fetchCategories(search);
      setCategories(page);
      setState('NOT_LOADING');
    }
    catch {
      setState('ERROR');
    }
  };

  const continueSearch = async () => {
    if(state === 'LOADING') { return };
    setState('LOADING');
    try {
      const page = await fetchCategories(currentSearch, categories.cursor);
      const nextCategories = {
        data: [...categories.data, ...page.data],
        cursor: page.cursor,
      };
      setCategories(nextCategories);
      setState('NOT_LOADING');
    }
    catch {
      setState('ERROR');
    }
  };

  const clearSearch = async () => {
    if(state === 'LOADING') { return };
    setState('LOADING');
    setSearch('');
    setCurrentSearch('');
    setCategories({ data: [] });
    try {
      const page = await fetchCategories('');
      setCategories(page);
      setState('NOT_LOADING');
    }
    catch {
      setState('ERROR');
    }
  };

  const submit = (event: FormEvent) => {
    event.preventDefault();
    startSearch();
  };

  useEffect(() => {
    startSearch(); 
  }, []);

  const cards = categories.data.map(category => {
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
      <form onSubmit={submit}>
        <div className="search-wrapper">
          <input
            className="search"
            value={search}
            placeholder="Search Categories"
            onChange={ (event: ChangeEvent<HTMLInputElement>) => setSearch(event.target.value) }
          />
          <button type="button" className="cross" onClick={clearSearch}>
            <img src="images/icons/cross.svg"/>
          </button>
        </div>
      </form>
      <ul className="card-list">{cards}</ul>
      { state === 'LOADING' && <img src="images/icons/loading.svg" className="spinner"/> }
      {
        state === 'NOT_LOADING' &&
        categories.cursor &&
        <button className="primary" onClick={continueSearch}>Load More</button>
      }
      {
        state === 'NOT_LOADING' &&
        categories.data.length == 0 &&
        <span>No categories found</span>
      }
      {
        state === 'ERROR' &&
        <span className="error">An error occurred. Please try again.</span>
      }
    </div>
  )
}


