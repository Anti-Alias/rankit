import { FormEvent, useState, useEffect } from 'react';
import "./SearchInput.css";

type Props = {
  search: (search: string) => Promise<void>,
  placeholder?: string,
};

export function SearchInput(props: Props) {
  const { search: searchFunc, placeholder } = props;
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(false);
  const load = async (searchValue: string) => {
    if (loading) { return };
    setLoading(true);
    await searchFunc(searchValue);
    setLoading(false);
  };
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    await load(search);
  };
  const clearSearch = async () => {
    if(search.length == 0) { return };
    setSearch("");
    await load("");
  };
  useEffect(() => { load("") }, []);

  return (
    <div className="SearchInput">
      <form onSubmit={submit}>
        <div className="wrapper">
          <input
            name="search"
            placeholder={placeholder}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
          <button type="button" className="cross" onClick={clearSearch}>
            <img src="images/icons/cross.svg" className="cross-image"/>
          </button>
        </div>
      </form>
      {loading ? <img src="images/icons/loading.svg" className="spinner" /> : null}
    </div>
  )
}

