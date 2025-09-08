import { writable } from "svelte/store";
import { teamsApi, handleApiError, type Team } from "../api";

interface TeamsState {
  teams: Team[];
  loading: boolean;
  error: string | null;
  selectedTeam: Team | null;
}

const createTeamsStore = () => {
  const { subscribe, update } = writable<TeamsState>({
    teams: [],
    loading: false,
    error: null,
    selectedTeam: null,
  });

  return {
    subscribe,

    async fetchAll() {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const teams = await teamsApi.getAll();

        update((state) => ({
          ...state,
          teams,
          loading: false,
        }));

        return teams;
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    async fetchById(id: string) {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const team = await teamsApi.getById(id);

        update((state) => ({
          ...state,
          selectedTeam: team,
          loading: false,
        }));

        return team;
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    async create(teamData: Partial<Team>) {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const newTeam = await teamsApi.create(teamData);

        update((state) => ({
          ...state,
          teams: [...state.teams, newTeam],
          loading: false,
        }));

        return newTeam;
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    async update(id: string, teamData: Partial<Team>) {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        const updatedTeam = await teamsApi.update(id, teamData);

        update((state) => ({
          ...state,
          teams: state.teams.map((team) =>
            team.id === id ? updatedTeam : team,
          ),
          selectedTeam:
            state.selectedTeam?.id === id ? updatedTeam : state.selectedTeam,
          loading: false,
        }));

        return updatedTeam;
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    async delete(id: string) {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        await teamsApi.delete(id);

        update((state) => ({
          ...state,
          teams: state.teams.filter((team) => team.id !== id),
          selectedTeam:
            state.selectedTeam?.id === id ? null : state.selectedTeam,
          loading: false,
        }));
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    async addMember(id: string, username: string, role: string = "member") {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        await teamsApi.addMember(id, username, role);

        // Refresh the team to get updated members
        const updatedTeam = await teamsApi.getById(id);

        update((state) => ({
          ...state,
          teams: state.teams.map((team) =>
            team.id === id ? updatedTeam : team,
          ),
          selectedTeam:
            state.selectedTeam?.id === id ? updatedTeam : state.selectedTeam,
          loading: false,
        }));
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    async removeMember(id: string, userId: string) {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        await teamsApi.removeMember(id, userId);

        // Refresh the team to get updated members
        const updatedTeam = await teamsApi.getById(id);

        update((state) => ({
          ...state,
          teams: state.teams.map((team) =>
            team.id === id ? updatedTeam : team,
          ),
          selectedTeam:
            state.selectedTeam?.id === id ? updatedTeam : state.selectedTeam,
          loading: false,
        }));
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    async updateMemberRole(id: string, userId: string, role: string) {
      update((state) => ({ ...state, loading: true, error: null }));

      try {
        await teamsApi.updateMemberRole(id, userId, role);

        // Refresh the team to get updated members
        const updatedTeam = await teamsApi.getById(id);

        update((state) => ({
          ...state,
          teams: state.teams.map((team) =>
            team.id === id ? updatedTeam : team,
          ),
          selectedTeam:
            state.selectedTeam?.id === id ? updatedTeam : state.selectedTeam,
          loading: false,
        }));
      } catch (error) {
        const errorMessage = handleApiError(error);
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage,
        }));
        throw new Error(errorMessage);
      }
    },

    clearSelected() {
      update((state) => ({ ...state, selectedTeam: null }));
    },

    clearError() {
      update((state) => ({ ...state, error: null }));
    },
  };
};

export const teamsStore = createTeamsStore();
