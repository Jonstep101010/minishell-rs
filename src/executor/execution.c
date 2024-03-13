/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   execution.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: apeposhi <apeposhi@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/13 10:07:19 by apeposhi          #+#    #+#             */
/*   Updated: 2024/03/13 11:02:59 by apeposhi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "minishell.h"
#include "execution.h"
#include "struct.h"
#include <string.h>

void execute(t_shell *shell, int tmp)
{
	char	*path;

	path = ft_getpath(shell->owned_envp, shell->command[0]);
	dup2(tmp, STDIN_FILENO);
	close(tmp);
	// add dup2 stdin
	// add dup2 stdout
	execve(path, shell->command, shell->owned_envp);
	// init exitstatus
	// print error
}

// commands, path, environment
int	exec(t_shell *shell, char **env)
{
	int	tmp;
	int	fd[2];

	tmp = dup(STDIN_FILENO);
	while (1)
	{
		if (<last_cmd>)
		{
			if (fork() == 0) {
			execute(shell, tmp);
			eprint("Something went wrong in execution"); //need review on this
		}
		close(tmp);
		while (waitpid(-1, NULL, WUNTRACED) != -1);
		tmp = dup(STDIN_FILENO);
		}
		else if (<pipe_cmd>)
		{
		pipe(fd);
		if (fork() == 0) {
			close(fd[0]);
			dup2(fd[1], STDOUT_FILENO);
			close(fd[1]);
			execute(shell, tmp);
			eprint("Something went wrong in execution"); //need review on this
		}
			close(fd[1]);
			close(tmp);
			tmp = fd[0];
		}
	}
	close(tmp);
}
