import { groupAPI } from '../ipc/group';

/**
 * Group service - encapsulates all group-related IPC calls
 */
export const groupService = {
  /**
   * Create a new group
   * @param groupName - Name of the group
   * @param creatorUid - User ID of the creator
   * @param memberUids - List of member user IDs to add
   * @returns The newly created group ID
   */
  async createGroup(groupName: string, creatorUid: number, memberUids: number[]) {
    return await groupAPI.createGroup(groupName, creatorUid, memberUids);
  },

  /**
   * Get group information
   * @param gid - Group ID
   * @returns Group information
   */
  async getGroupInfo(gid: number) {
    return await groupAPI.getGroupInfo(gid);
  },

  /**
   * Get all members of a group
   * @param gid - Group ID
   * @returns List of group members
   */
  async getGroupMembers(gid: number) {
    return await groupAPI.getGroupMembers(gid);
  },

  /**
   * Add a member to a group
   * @param gid - Group ID
   * @param memberUid - User ID of the member to add
   * @param role - Member role (0=Member, 1=Admin, 2=Owner)
   */
  async addGroupMember(gid: number, memberUid: number, role: number) {
    return await groupAPI.addGroupMember(gid, memberUid, role);
  },

  /**
   * Remove a member from a group
   * @param gid - Group ID
   * @param memberUid - User ID of the member to remove
   */
  async removeGroupMember(gid: number, memberUid: number) {
    return await groupAPI.removeGroupMember(gid, memberUid);
  },

  /**
   * Update a member's role in a group
   * @param gid - Group ID
   * @param memberUid - User ID of the member
   * @param role - New role (0=Member, 1=Admin, 2=Owner)
   */
  async updateMemberRole(gid: number, memberUid: number, role: number) {
    return await groupAPI.updateMemberRole(gid, memberUid, role);
  },

  /**
   * Get all groups the user is a member of
   * @param userUid - User ID
   * @returns List of groups
   */
  async getUserGroups(userUid: number) {
    return await groupAPI.getUserGroups(userUid);
  },

  /**
   * Update group information
   * @param gid - Group ID
   * @param groupName - New group name
   * @param desc - New group description
   */
  async updateGroupInfo(gid: number, groupName: string, desc: string) {
    return await groupAPI.updateGroupInfo(gid, groupName, desc);
  },

  /**
   * Delete a group
   * @param gid - Group ID to delete
   */
  async deleteGroup(gid: number) {
    return await groupAPI.deleteGroup(gid);
  },
};
