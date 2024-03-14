/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   execution.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: apeposhi <apeposhi@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/13 10:07:19 by apeposhi          #+#    #+#             */
/*   Updated: 2024/03/14 14:03:10 by apeposhi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// #include "minishell.h"
// #include "execution.h"
// #include "struct.h"
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <stdio.h>
#include <fcntl.h>

typedef struct s_shell
{
	char	**command;
	char	**owned_envp;
	int		input_redirect;
	char	*input_file;
	int		output_redirect;
	char	*output_file;
	int		error_redirect;
	char	*error_file;
	int		type;
}	t_shell;

size_t	ft_strlen(const char *str)
{
	size_t	i;

	i = 0;
	while (str[i] != '\0')
		i++;
	return (i);
}

size_t	ft_strlcpy(char *dst, const char *src, size_t size)
{
	size_t	srclen;
	size_t	x;

	srclen = ft_strlen(src);
	if (!size)
		return (srclen);
	x = -1;
	while ((src[++x] != '\0') && (x < size - 1))
	{
		dst[x] = src[x];
	}
	if (!src[x] || (x == size - 1))
		dst[x] = 0;
	return (srclen);
}

size_t	ft_strlcat(char *dst, const char *src, size_t dstsize)
{
	size_t	i;
	size_t	srclen;
	size_t	dstlen;

	dstlen = ft_strlen(dst);
	if (dstsize <= dstlen)
		return (dstsize + ft_strlen(src));
	srclen = ft_strlen(src);
	i = -1;
	while ((++i < srclen) && (i + dstlen < dstsize - 1))
	{
		dst[i + dstlen] = src[i];
	}
	dst[i + dstlen] = '\0';
	return (dstlen + srclen);
}

char	*ft_strjoin(char const *s1, char const *s2)
{
	char	*strcat;
	size_t	s1_len;
	size_t	s2_len;

	s1_len = ft_strlen(s1);
	s2_len = ft_strlen(s2);
	strcat = malloc(s1_len + s2_len + 1);
	if (!strcat)
		return (NULL);
	ft_strlcpy(strcat, s1, s1_len + 1);
	return (ft_strlcat(strcat, s2, s1_len + s2_len + 1), strcat);
}
 //////////////////////////////////////////////////////////////////////////////////////////
char	*ft_strmanip(char *path, char *f_cmd)
{
	char	*path_data;
	char	*cmd;

	path_data = ft_strjoin(path, "/");
	cmd = ft_strjoin(path_data, f_cmd);
	free(path_data);
	return (cmd);
}

int arrlen(char** arr)
{
    int it;

    it = -1;
    if (arr)
        while (arr[++it])
            ;
    return (it);
}

char** arrdup(char** arr)
{
    int len;
    char** new_arr;

    len = arrlen(arr);
    if (len == -1)
        return (NULL);
    new_arr = malloc(len * sizeof(char*));
    if (new_arr == NULL)
        return NULL;
    for (int i = 0; i < len; i++)
    {
        new_arr[i] = strdup(arr[i]);
        if (new_arr[i] == NULL)
        {
            for (int j = 0; j < i; j++)
                free(new_arr[j]);
            free(new_arr);
            return NULL;
        }
    }
    return (new_arr);
}

char *ft_arrcmp(char **arr, char * str)
{
	int i;

	i = 0;
	while (arr[i])
	{
		if (!strncmp(arr[i], str, strlen(str)))
			break ;
		i++;
	}
	return (arr[i]);
}

char	*ft_getpath(char **env, char *f_cmd)
{
	char	*path;
	char	*tmp;
	char	*s_tmp;

 	s_tmp = NULL;
	path = ft_arrcmp(env, "PATH=");
	if (!path)
		return (NULL);
	path += 5;
	tmp = path;
	while (access(s_tmp, X_OK)) {
		if (s_tmp)
			free(s_tmp);
		path = tmp;
		while (*tmp && *tmp != ':')
			tmp++;
		*tmp++ = '\0';
 		s_tmp = ft_strmanip(path, f_cmd);
	}
	return (s_tmp);
}

/////////////////////////////////////////////////////////////////////////////////

// HANDLING REDIRECTIONS |||| WIP

// void execute(t_shell *shell, int tmp)
// {
// 	char	*path;

// 	path = ft_getpath(shell->owned_envp, shell->command[0]);
// 	dup2(tmp, STDIN_FILENO);
// 	close(tmp);
// 	// add dup2 stdin
// 	// add dup2 stdout
// 	execve(path, shell->command, shell->owned_envp);
// 	// init exitstatus
// 	// print error
// }

void execute(t_shell *shell, int tmp)
{
    char *path;

    path = ft_getpath(shell->owned_envp, shell->command[0]);
    
	if (shell->input_redirect)
	{
		int input_fd;

		input_fd = open(shell->input_file, O_RDONLY);
		if (input_fd < 0)
		{
			perror("Failed to open input file for redirection.");
			exit(EXIT_FAILURE);
		}
		dup2(input_fd, STDIN_FILENO);
		close(input_fd);
	}
	else
		dup2(tmp, STDIN_FILENO);
	if (shell->output_redirect) {
		int	output_fd;

		output_fd = open(shell->output_file, O_WRONLY | O_CREAT | O_TRUNC, 0666);
		if (output_fd < 0)
		{
			perror("Failed to open output file for redirection");
			exit(EXIT_FAILURE);
		}
		dup2(output_fd, STDOUT_FILENO);
		close(output_fd);
	}
	if (shell->error_redirect)
	{
		int	error_fd;

		error_fd = open(shell->error_file, O_WRONLY | O_CREAT | O_TRUNC, 0666);
		if (error_fd < 0)
		{
			perror("Failed to open error file for redirection.");
			exit(EXIT_FAILURE);
		}
		dup2(error_fd, STDERR_FILENO);
		close(error_fd);
	}
	close(tmp);
	execve(path, shell->command, shell->owned_envp);
	perror("execve failed");
	exit(EXIT_FAILURE);
}

// commands, path, environment
int	exec(t_shell *shell, char **env)
{
	int	tmp;
	int	fd[2];

	tmp = dup(STDIN_FILENO);
	// while (1)
	// {
		if (!shell->type)
		{
			if (fork() == 0) {
				execute(shell, tmp);
			// eprint("Something went wrong in execution"); //need review on this
			}
			close(tmp);
			while (waitpid(-1, NULL, WUNTRACED) != -1);
			tmp = dup(STDIN_FILENO);
		}
		else if (shell->type)
		{
			pipe(fd);
			if (fork() == 0) {
				close(fd[0]);
				dup2(fd[1], STDOUT_FILENO);
				close(fd[1]);
				execute(shell, tmp);
				// eprint("Something went wrong in execution"); //need review on this
			}
			close(fd[1]);
			close(tmp);
			tmp = fd[0];
		}
	// }
	close(tmp);
	return (0);
}

int main(int argc, char **argv, char **envp)
{
	t_shell shell;
// dprintf(2, "test\n");
	shell.owned_envp = arrdup(envp);
	shell.command = malloc(3 * sizeof(char *));
	shell.command[0] = strdup("ls");
	shell.command[1] = strdup("-l");
	shell.command[2] = NULL;
	
	shell.type = 0;
	
	exec(&shell, envp);
	return 0;
}