import { Fragment, useEffect, useState } from "react";
import { useParams } from "react-router"
import { fetchThingWithRankings, Ranking, ThingWithRankings } from "../../utils/thing";
import './Thing.css';
import { Score } from "../widgets/Score";

type State = 
  { kind: 'loading' } |
  { kind: 'error' } |
  { kind: 'finished', rankedThing: ThingWithRankings};

export function Thing() {
  const params = useParams();
  const id = checkNaN(parseInt(params.id as string));
  const [state, setState] = useState<State>({ kind: 'loading' });
  const load = async () => {
    try {
      const rankedThing = await fetchThingWithRankings(id);
      setState({ kind: 'finished', rankedThing });
    }
    catch {
      setState({ kind: 'error' });
    }
  };

  useEffect(() => {
    load();
  }, []);

  switch(state.kind) {
    case 'loading':   return <img src="/images/icons/loading.svg" className="spinner" />
    case 'error':     return <span className="error">Failed to load data</span>
    case 'finished':  return (
      <div>
        <h1>{state.rankedThing.thing.name}</h1>
        <img className="thing-image" src={state.rankedThing.thing.image}/>
        {
          state.rankedThing.rankings.length > 0 &&
          <Fragment>
            <h2>Rankings</h2>
            <CategoriesOfThing rankings={state.rankedThing.rankings}/>
          </Fragment>
        }
      </div>
    )
  }
}

function CategoriesOfThing({ rankings }: { rankings: Ranking[] }) {
  const rankingList = rankings.map(ranking => (
    <li key={ranking.category.id}>
      <button type="button" className="card">
        <div className="image-wrapper">
          <img className="card-image" src={ranking.category.image} />
          <Score winRate={ranking.winRate}/>
        </div>
        {ranking.category.name}
      </button>
    </li>
  ));
  return <ul className="card-list">{rankingList}</ul>;
}

function checkNaN(value: number): number {
  if(isNaN(value)) {
    throw new Error('Numeric value was NaN');
  }
  return value;
}
