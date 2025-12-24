import classNames from 'classnames';
import { useEffect, useState } from 'react';
import { contractAddress } from 'config';
import { WidgetType } from 'types/widget.types';
import { DashboardHeader, LeftPanel, Widget } from './components';
import styles from './dashboard.styles';
import { CreateTask, TaskBoard, Transactions } from './widgets';

const dashboardWidgets: WidgetType[] = [
  {
    title: 'Create Task',
    widget: CreateTask,
    description: 'Create a new distributed computing task',
    reference: 'https://github.com/multiversx/mx-template-dapp'
  },
  {
    title: 'Task Board',
    widget: TaskBoard,
    description: 'View available distributed computing tasks',
    reference: 'https://github.com/multiversx/mx-template-dapp'
  },
  // Removed less relevant template widgets: Sign message, Native auth, Batch Transactions
  {
    title: 'Transactions (All)',
    widget: () => <Transactions identifier='transactions-all' />,
    description: 'List transactions for the connected account',
    reference:
      'https://api.multiversx.com/#/accounts/AccountController_getAccountTransactions'
  }
];

export const Dashboard = () => {
  const [isMobilePanelOpen, setIsMobilePanelOpen] = useState(false);

  useEffect(() => {
    if ('scrollRestoration' in history) {
      history.scrollRestoration = 'manual';
    }
  }, []);

  return (
    <div className={styles.dashboardContainer}>
      <div
        className={classNames(
          styles.mobilePanelContainer,
          styles.desktopPanelContainer
        )}
      >
        <LeftPanel
          isOpen={isMobilePanelOpen}
          setIsOpen={setIsMobilePanelOpen}
        />
      </div>

      <div
        style={{ backgroundImage: 'url(/background.svg)' }}
        className={classNames(styles.dashboardContent, {
          [styles.dashboardContentMobilePanelOpen]: isMobilePanelOpen
        })}
      >
        <DashboardHeader />

        <div className={styles.dashboardWidgets}>
          {dashboardWidgets.map((element) => (
            <Widget key={element.title} {...element} />
          ))}
        </div>
      </div>
    </div>
  );
};
