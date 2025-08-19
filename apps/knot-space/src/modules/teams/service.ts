import { prisma } from '../../lib/prisma';
import { CreateTeamRequest, AddTeamMemberRequest } from '../../types';

class TeamsService {
  private async findTeamByIdentifier(teamIdentifier: string) {
    // Check if the identifier is a UUID or a team name
    const isUuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(teamIdentifier);
    
    const team = await prisma.team.findUnique({
      where: isUuid ? { id: teamIdentifier } : { name: teamIdentifier }
    });

    if (!team) {
      throw new Error('Team not found');
    }

    return team;
  }
  async createTeam(data: CreateTeamRequest, ownerId: string) {
    const existingTeam = await prisma.team.findUnique({
      where: { name: data.name }
    });

    if (existingTeam) {
      throw new Error('Team name already exists');
    }

    const team = await prisma.team.create({
      data: {
        name: data.name,
        description: data.description,
        ownerId,
      },
      include: {
        owner: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        }
      }
    });

    // Add owner as team member
    await prisma.teamMember.create({
      data: {
        teamId: team.id,
        userId: ownerId,
        role: 'owner',
      }
    });

    // Return the team with members included to match the expected structure
    const teamWithMembers = await prisma.team.findUnique({
      where: { id: team.id },
      include: {
        owner: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        },
        members: {
          include: {
            user: {
              select: {
                id: true,
                username: true,
                email: true,
                createdAt: true,
              }
            }
          }
        }
      }
    });

    return teamWithMembers;
  }

  async listTeams(userId?: string) {
    if (userId) {
      // Get teams where user is a member
      const teamMembers = await prisma.teamMember.findMany({
        where: { userId },
        include: {
          team: {
            include: {
              owner: {
                select: {
                  id: true,
                  username: true,
                  email: true,
                  createdAt: true,
                }
              },
              _count: {
                select: {
                  members: true,
                  packages: true,
                }
              },
              members: {
                include: {
                  user: {
                    select: {
                      id: true,
                      username: true,
                      email: true,
                      createdAt: true,
                    }
                  }
                }
              }
            }
          }
        }
      });

      return teamMembers.map(tm => ({
        ...tm.team,
        memberRole: tm.role,
      }));
    } else {
      // Public teams list
      return await prisma.team.findMany({
        include: {
          owner: {
            select: {
              id: true,
              username: true,
              email: true,
              createdAt: true,
            }
          },
          _count: {
            select: {
              members: true,
              packages: true,
            }
          },
          members: {
            include: {
              user: {
                select: {
                  id: true,
                  username: true,
                  email: true,
                  createdAt: true,
                }
              }
            }
          }
        },
        orderBy: { createdAt: 'desc' }
      });
    }
  }

  async getTeam(teamIdentifier: string, userId?: string) {
    const team = await this.findTeamByIdentifier(teamIdentifier);
    
    // Get team with all details
    const fullTeam = await prisma.team.findUnique({
      where: { id: team.id },
      include: {
        owner: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        },
        _count: {
          select: {
            members: true,
            packages: true,
          }
        },
        members: {
          include: {
            user: {
              select: {
                id: true,
                username: true,
                email: true,
                createdAt: true,
              }
            }
          }
        }
      }
    });

    if (!fullTeam) {
      throw new Error('Team not found');
    }

    let memberRole = null;
    if (userId) {
      const member = await prisma.teamMember.findUnique({
        where: {
          teamId_userId: {
            teamId: fullTeam.id,
            userId,
          }
        }
      });
      memberRole = member?.role || null;
    }

    return {
      ...fullTeam,
      memberRole,
    };
  }

  async getTeamMembers(teamIdentifier: string) {
    const team = await this.findTeamByIdentifier(teamIdentifier);

    const members = await prisma.teamMember.findMany({
      where: { teamId: team.id },
      include: {
        user: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        }
      },
      orderBy: [
        { role: 'asc' }, // owners first, then admins, then members
        { joinedAt: 'asc' }
      ]
    });

    return members;
  }

  async addTeamMember(teamIdentifier: string, data: AddTeamMemberRequest, currentUserId: string) {
    const team = await this.findTeamByIdentifier(teamIdentifier);
    
    // Check if current user has permission to add members
    const currentMember = await prisma.teamMember.findUnique({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId: currentUserId,
        }
      }
    });

    if (!currentMember || (currentMember.role !== 'owner' && currentMember.role !== 'admin')) {
      throw new Error('Insufficient permissions to add team members');
    }

    // Find user by username
    const user = await prisma.user.findUnique({
      where: { username: data.username }
    });

    if (!user) {
      throw new Error('User not found');
    }

    // Check if user is already a member
    const existingMember = await prisma.teamMember.findUnique({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId: user.id,
        }
      }
    });

    if (existingMember) {
      throw new Error('User is already a team member');
    }

    const newMember = await prisma.teamMember.create({
      data: {
        teamId: team.id,
        userId: user.id,
        role: data.role,
      },
      include: {
        user: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        }
      }
    });

    return newMember;
  }

  async removeTeamMember(teamIdentifier: string, userId: string, currentUserId: string) {
    const team = await this.findTeamByIdentifier(teamIdentifier);
    
    // Check if current user has permission to remove members
    const currentMember = await prisma.teamMember.findUnique({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId: currentUserId,
        }
      }
    });

    if (!currentMember || (currentMember.role !== 'owner' && currentMember.role !== 'admin')) {
      throw new Error('Insufficient permissions to remove team members');
    }

    const memberToRemove = await prisma.teamMember.findUnique({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId,
        }
      }
    });

    if (!memberToRemove) {
      throw new Error('User is not a team member');
    }

    // Cannot remove team owner
    if (memberToRemove.role === 'owner') {
      throw new Error('Cannot remove team owner');
    }

    await prisma.teamMember.delete({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId,
        }
      }
    });
  }

  async updateMemberRole(teamIdentifier: string, userId: string, newRole: string, currentUserId: string) {
    const team = await this.findTeamByIdentifier(teamIdentifier);
    
    // Check if current user has permission to update member roles
    const currentMember = await prisma.teamMember.findUnique({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId: currentUserId,
        }
      }
    });

    if (!currentMember || (currentMember.role !== 'owner' && currentMember.role !== 'admin')) {
      throw new Error('Insufficient permissions to update member roles');
    }

    const memberToUpdate = await prisma.teamMember.findUnique({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId,
        }
      }
    });

    if (!memberToUpdate) {
      throw new Error('User is not a team member');
    }

    // Cannot change team owner role
    if (memberToUpdate.role === 'owner') {
      throw new Error('Cannot change team owner role');
    }

    // Update the member role
    const updatedMember = await prisma.teamMember.update({
      where: {
        teamId_userId: {
          teamId: team.id,
          userId,
        }
      },
      data: {
        role: newRole,
      },
      include: {
        user: {
          select: {
            id: true,
            username: true,
            email: true,
            createdAt: true,
          }
        }
      }
    });

    return updatedMember;
  }

  async deleteTeam(teamIdentifier: string, currentUserId: string) {
    const team = await this.findTeamByIdentifier(teamIdentifier);

    if (team.ownerId !== currentUserId) {
      throw new Error('Only team owner can delete the team');
    }

    await prisma.team.delete({
      where: { id: team.id }
    });
  }
}

export const teamsService = new TeamsService();