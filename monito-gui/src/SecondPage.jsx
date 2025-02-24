
import { useLocation, useParams } from "react-router";
import TabsPage from "./TabsPage";

const SecondPage = () => {
    const location = useLocation();
    const monitors = location.state;

    return (
      <div className='w-dvw h-dvh'>
        <TabsPage items={monitors} />
      </div>
    );
}

export default SecondPage;