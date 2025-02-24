import { RouterProvider } from "react-router";
import BasicWindow from "./TitleBar";
import routers from "./utils/router";

const App = () => {
    return (
      <>
        <BasicWindow>
          <RouterProvider router={routers} />
        </BasicWindow>
      </>
    );
}

export default App;