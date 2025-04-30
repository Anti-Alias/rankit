import { Page } from "./page";

export interface Category {
  id: number,
  name: string,
  image: string,
}

export async function fetchCategories(search: string): Promise<Page<Category>> {
  await sleep(1000);
  const data = allCategories.data.filter(cat => cat.name.includes(search));
  return { data };
}

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}


const allCategories: Page<Category> = {
  data: [
    {
      id: 1,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 2,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 3,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 4,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 5,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 6,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 7,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 8,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 9,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 10,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 11,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 12,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 13,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 14,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 15,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 16,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 17,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 18,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 19,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 20,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 21,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 22,
      name: 'Movies',
      image: 'movies.svg',
    },
    {
      id: 23,
      name: 'TV Shows',
      image: 'tv.svg',
    },
    {
      id: 24,
      name: 'Games',
      image: 'games.svg',
    },
    {
      id: 25,
      name: 'Games',
      image: 'games.svg',
    },
  ],
  cursor: "stub",
};
