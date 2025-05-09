import { fetchCategories } from "../../utils/category";
import { InfiniteScroll } from "../widgets/InfiniteScroll";

export function CategoryList() {
  return <InfiniteScroll
    searchPlaceholder="Search Categories"
    notFoundText="Categories not found"
    fetcher={fetchCategories}
  />;
}

