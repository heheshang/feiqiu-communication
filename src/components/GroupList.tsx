import React from 'react';
import { useGroupStore } from '../store/groupStore';
import './GroupList.less';

interface GroupListProps {
  onSelectGroup?: (groupId: number) => void;
}

export const GroupList: React.FC<GroupListProps> = ({ onSelectGroup }) => {
  const groups = useGroupStore((state) => state.groups);

  return (
    <div className="group-list">
      <h2>Groups</h2>
      {groups.length === 0 ? (
        <p>No groups yet</p>
      ) : (
        <ul>
          {groups.map((group) => (
            <li key={group.gid} onClick={() => onSelectGroup?.(group.gid)} className="group-item">
              {group.group_name}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};
