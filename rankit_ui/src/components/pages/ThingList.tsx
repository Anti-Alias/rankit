import { useNavigate } from "react-router";
import { fetchThingPage } from "../../utils/thing";
import { InfiniteScroll } from "../widgets/InfiniteScroll";

export function ThingList() {
  const navigate = useNavigate();
  return <InfiniteScroll
    searchPlaceholder = "Search Things"
    notFoundText = "Things not found"
    fetcher = { fetchThingPage }
    onItemClick = { id => navigate(`/things/${id}`) }
  />;
}

