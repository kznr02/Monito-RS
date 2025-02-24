import { useNavigate } from "react-router";
import ConfigPage from "./ConfigPage";

const TabsPage = ({ items }) => {
  const navigate = useNavigate();

  const jumpToFront = () => {
    navigate('/', {});
  }

    return (
      <div
        role='tablist'
        className='tabs tabs-border w-dvw h-[calc(100dvh-var(--title-height))] items-center'
      >
        <button className="btn btn-primary btn-xs ml-3 z-1" onClick={() => jumpToFront()}>返回</button>
        {items.monitors.map((m, index) => (
          <>
              <input
                key={index}
                type='radio'
                name='SettingTab'
                role='tab'
                className='tab text-primary hover:text-primary'
                aria-label={m.name}
                defaultChecked={items.select == index ? true : false}
              />
            <div
              role='tabpanel'
              className='tab-content border-base-300 bg-base-100 rounded-box p-5'
            >
              <ConfigPage item={m} select={index} />
            </div>
          </>
        ))}
      </div>
    );
}

export default TabsPage;