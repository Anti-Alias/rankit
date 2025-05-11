import { Category } from "./category";
import { Thing } from "./thing";

export interface RawRanking {
  thingId: number,
  categoryId: number,
  winRate: number,
}

export function allThings(): Thing[] {
  const result: Thing[] = [];
  for (let i = 0; i < 100; i += 4) {
    result.push({
      id: i,
      name: `The Godfather ${i}`,
      image: '/images/things/godfather.jpg',
    });
    result.push({
      id: i + 1,
      name: `Family Guy ${i + 1}`,
      image: '/images/things/family-guy.jpg',
    });
    result.push({
      id: i + 2,
      name: `Rice ${i + 2}`,
      image: '/images/things/rice.jpg',
    });
    result.push({
      id: i + 3,
      name: `Apples ${i + 3}`,
      image: '/images/things/apples.jpg',
    });
  }
  return result;
}

export function allCategories(): Category[] {
  const result: Category[] = [];
  for (let i = 0; i < 100; i += 4) {
    result.push({
      id: i,
      name: `Movies ${i}`,
      image: '/images/categories/movies.jpg',
    });
    result.push({
      id: i + 1,
      name: `TV Shows ${i + 1}`,
      image: '/images/categories/tv_shows.jpg',
    });
    result.push({
      id: i + 2,
      name: `Food ${i + 2}`,
      image: '/images/categories/food.jpg',
    });
    result.push({
      id: i + 3,
      name: `Fruit ${i + 3}`,
      image: '/images/categories/fruit.png',
    });
  }
  return result;
}

// Thing ids:     0 = The Godfather,  1 = Family guy, 2 = Rice, 3 = Apples
// Category ids:  0 = Movies,         1 = TV Shows,   2 = Food, 3 = Fruit 
export function rawRankings(): RawRanking[] {
  return [
    {
      thingId: 0,
      categoryId: 0,
      winRate: .9,
    },
    {
      thingId: 1,
      categoryId: 1,
      winRate: 0.7,
    },
    {
      thingId: 2,
      categoryId: 2,
      winRate: .9,
    },
    {
      thingId: 3,
      categoryId: 2,
      winRate: .4,
    },
    {
      thingId: 3,
      categoryId: 3,
      winRate: .8,
    },
  ];
}

