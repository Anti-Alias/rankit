import { ScrollItem } from "../components/widgets/InfiniteScroll";
import { Category } from "./category";
import { allCategories, allThings, rawRankings } from "./db";
import { Page } from "./page";

export interface Thing extends ScrollItem {};
export interface ThingWithRankings { thing: Thing, rankings: Ranking[] }
export interface Ranking { category: Category, winRate: number }


const sleepTime: number = 1000;
const pageSize: number = 32;


export async function fetchThingPage(search: string, cursor?: string): Promise<Page<Thing>> {
  await new Promise(resolve => setTimeout(resolve, sleepTime));
  const data = allThings().filter(thing => thing.name.includes(search));
  const curs = cursor ? parseInt(cursor) : 0;
  const nextCursor = curs + pageSize;
  if (data.length - 1 > nextCursor) {
    const splicedData = data.splice(curs, nextCursor - curs);
    return { data: splicedData, cursor: nextCursor.toString() };
  }
  else {
    const splicedData = data.splice(curs, nextCursor - curs);
    return { data: splicedData }
  }
}

export async function fetchThingWithRankings(thingId: number): Promise<ThingWithRankings> {
  await new Promise(resolve => setTimeout(resolve, sleepTime));
  const things              = allThings();
  const categories          = allCategories();
  const rankings: Ranking[] = rawRankings()
    .filter(rawRanking => rawRanking.thingId === thingId)
    .sort((a, b) => b.winRate - a.winRate)
    .map(rawRanking => ({
      category: categories[rawRanking.categoryId],
      winRate: rawRanking.winRate,
    }));
  console.log(rankings);
  return {
    thing: things[thingId],
    rankings,
  }
}

