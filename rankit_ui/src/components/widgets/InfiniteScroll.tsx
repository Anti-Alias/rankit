
import { ChangeEvent, FormEvent, useEffect, useState } from "react";
import { Page } from "../../utils/page";
import "./InfiniteScroll.css";

type State = 'INIT' | 'NOT_LOADING' | 'LOADING' | 'ERROR';

export type InfiniteScrollProps = {
  searchPlaceholder?: string,
  notFoundText?: string,
  fetcher: (search: string, cursor?: string) => Promise<Page<ScrollItem>>,
}

/// A growable list of items, fetched from an api using cursor-based pagination.
export function InfiniteScroll(props: InfiniteScrollProps) {

  const searchPlaceholder = props.searchPlaceholder ? props.searchPlaceholder : "Search";
  const fetcher = props.fetcher;
  const notFoundText = props.notFoundText ? props.notFoundText : "Not Found";

  const [items, setItems] = useState<Page<ScrollItem>>({ data: [] });
  const [search, setSearch] = useState('');
  const [currentSearch, setCurrentSearch] = useState('');
  const [state, setState] = useState<State>('INIT');

  const startSearch = async () => {
    if (state === 'LOADING') { return };
    setState('LOADING');
    setItems({ data: [] });
    setCurrentSearch(search);
    try {
      const page = await fetcher(search);
      setItems(page);
      setState('NOT_LOADING');
    }
    catch {
      setState('ERROR');
    }
  };

  const continueSearch = async () => {
    if (state === 'LOADING') { return };
    setState('LOADING');
    try {
      const page = await fetcher(currentSearch, items.cursor);
      const nextPage = {
        data: [...items.data, ...page.data],
        cursor: page.cursor,
      };
      setItems(nextPage);
      setState('NOT_LOADING');
    }
    catch {
      setState('ERROR');
    }
  };

  const clearSearch = async () => {
    setSearch('');
    if (state === 'LOADING') { return };
    setState('LOADING');
    setItems({ data: [] });
    setCurrentSearch('');
    try {
      const page = await fetcher('');
      setItems(page);
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

  const cards = items.data.map(scrollItem => {
    return (
      <li key={scrollItem.id}>
        <button type="button" className="card">
          <img className="card-image" src={scrollItem.image} />
          {scrollItem.name}
        </button>
      </li>
    )
  });

  return (
    <div className="InfiniteScroll">
      <form onSubmit={submit} className="search-form">
          <input
            value={search}
            placeholder={searchPlaceholder}
            onChange={(event: ChangeEvent<HTMLInputElement>) => setSearch(event.target.value)}
          />
          <button type="button" className="cross" onClick={clearSearch}>
            <img src="images/icons/cross.svg" />
          </button>
      </form>
      {
        cards.length > 0 &&
        <ul className="card-list">{cards}</ul>
      }
      {
        (state === 'INIT' || state == 'LOADING') &&
        <img src="images/icons/loading.svg" className="spinner" />}
      {
        state === 'NOT_LOADING' &&
        items.cursor &&
        <button className="primary" onClick={continueSearch}>Load More</button>
      }
      {
        state === 'NOT_LOADING' &&
        items.data.length == 0 &&
        <span className="message">{notFoundText}</span>
      }
      {
        state === 'ERROR' &&
        <span className="error message">An error occurred. Please try again.</span>
      }
    </div>
  )
}


/// Data for each card.
export interface ScrollItem {
  id: number,
  name: string,
  image: string,
}

